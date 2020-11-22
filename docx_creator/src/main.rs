use docx_rs::{
    AlignmentType, BreakType, Docx, DocxError, PageMargin, Paragraph, Run, RunFonts, Table,
    TableCell, TableCellContent, TableRow,
};
use fs_extra::file::move_file;
use fs_extra::file::CopyOptions;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::{create_dir, read_dir, read_to_string};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;
use std::str;
use Result::{Err, Ok};

pub fn main() {
    let cfg = read_config("config/config.json").unwrap();

    create_iib_delivery_structure(&cfg).expect("Error in create iib struct");
}

/// Reads configuration file
fn read_config<T>(path: T) -> io::Result<Config>
where
    T: AsRef<Path>,
{
    let contents = read_to_string(path).expect("Error opening config.json");
    let config: Config = serde_json::from_str(&contents).expect("Error while parsing config.json");

    Ok(config)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub path_to_iib_cmd: String,
    pub path_to_app: String,
    pub path_to_bar: String,
    pub path_to_out_props: String,
}

/// Reads properties to an attached bar file
fn call_mqsireadbar(cfg: &Config) -> Result<(), ()> {
    if cfg!(target_os = "windows") {
        Command::new(&cfg.path_to_iib_cmd)
            .args(&[
                "mqsireadbar",
                "-b",
                &cfg.path_to_bar,
                "-r",
                ">",
                &cfg.path_to_out_props,
            ])
            .output()
            .expect("failed to execute process");
    } else {
        panic!("Not supported OS")
    }

    Ok(())
}

/// Creates docx RN
fn create_rn_iib(cfg: &Config, props: (Vec<String>, Vec<String>)) -> Result<(), DocxError> {
    let docx_path = format!("{}\\rn\\_{}_APPL_RN.docx", &cfg.path_to_app, &cfg.app_name);
    let path = std::path::Path::new(&docx_path);
    let file = std::fs::File::create(&path).unwrap();
    let header_text = "Сервис NIB для работы с MAUL";
    let rn_txt = "RELEASE NOTES";
    let break_next_page = Paragraph::new().add_run(Run::new().add_break(BreakType::Page));
    let header_par = Paragraph::new()
        .add_run(
            Run::new()
                .add_break(BreakType::TextWrapping)
                .add_break(BreakType::TextWrapping)
                .add_break(BreakType::TextWrapping)
                .add_text(header_text)
                .size(44)
                .bold(),
        )
        .add_run(
            Run::new()
                .add_break(BreakType::TextWrapping)
                .add_break(BreakType::TextWrapping)
                .add_text(rn_txt)
                .fonts(RunFonts::new().ascii("Calibri Light"))
                .size(68)
                .bold()
                .underline("single underlined")
                .add_break(BreakType::Page),
        )
        .align(AlignmentType::Right);
    let change_history_table = create_empty_table(
        4,
        3,
        vec![
            "№ версии",
            "Дата",
            "Краткое описание изменений",
            "Разработчик",
        ],
    )
    .unwrap();

    let properties_table =
        create_empty_table(3, props.0.len(), vec!["Параметр", "Значение", "Описание"]).unwrap();
    let properties_table = fill_table(properties_table, props).unwrap();

    Docx::new()
        .add_paragraph(header_par)
        .add_table(change_history_table)
        .add_paragraph(break_next_page)
        .add_table(properties_table)
        .page_margin(PageMargin {
            bottom: 1132,
            header: 851,
            left: 1701,
            right: 850,
            footer: 992,
            gutter: 0,
            top: 1132,
        })
        .build()
        .pack(file)?;

    Ok(())
}

/// Creates an stucture of folders according to iib folder structure
fn create_iib_delivery_structure(cfg: &Config) -> io::Result<()> {
    let app_name = &cfg.app_name;
    let path = &cfg.path_to_app;
    let dir_file_names: Vec<_> = read_dir(path)
        .unwrap()
        .filter_map(|dir_entry| match dir_entry {
            Ok(dir_entry) => match dir_entry.file_name().to_str().unwrap() {
                "rn" => None,
                "src" => None,
                "build" => None,
                _ => Some(dir_entry),
            },
            Err(_) => None,
        })
        .collect();

    let src_folder = format!("{}{}", path, "\\src");
    let rn_folder = format!("{}{}", path, "\\rn");
    let build_folder = format!("{}{}", path, "\\build");
    create_dir(&rn_folder)
        .unwrap_or_else(|x| println!("Probably some directories already existed: {}", x));
    create_dir(&build_folder)
        .unwrap_or_else(|x| println!("Probably some directories already existed: {}", x));
    create_dir(&src_folder)
        .unwrap_or_else(|x| println!("Probably some directories already existed: {}", x));

    let app_folder = format!("{}\\{}{}", src_folder, app_name, "_APPL");
    //println!("{}", app_folder);
    let misc_folder = format!("{}\\{}{}", src_folder, app_name, "_MISC");
    create_dir(&app_folder)
        .unwrap_or_else(|x| println!("Probably some directories already existed: {}", x));
    create_dir(&misc_folder)
        .unwrap_or_else(|x| println!("Probably some directories already existed: {}", x));

    for i in dir_file_names {
        if i.file_type().unwrap().is_file() {
            move_file(
                i.path(),
                format!("{}\\{}", app_folder, i.file_name().to_str().unwrap()),
                &CopyOptions::default(),
            )
            .expect("Error while moving appl to a new folder");
        } else if i.file_type().unwrap().is_dir() {
            move_dir_through_cmd(
                &format!("{}\\{}", path, i.file_name().to_str().unwrap()),
                &app_folder,
            )
            .unwrap_or_else(|e| println!("Error during moving a directory: {}", e));
        }
    }

    let f = create_if_not_exist(&format!("{}{}", src_folder, "\\.gitignore"));
    match f {
        Ok(mut _f) => _f.write_fmt(format_args!(
            "{}\n{}\n{}\n{}\n",
            ".metadata", ".idea", "GeneratedBarFiles", "BARfiles"
        ))?,
        Err(e) => println!("Gitignore: {}", e),
    }
    match create_if_not_exist(&format!(
        "{}{}",
        src_folder, "\\createExtProjectsSymLinks.sh"
    )) {
        Ok(_) => println!("createExtProjectsSymLinks.sh has created!"),
        Err(e) => println!("createExtProjectsSymLinks.sh: {}", e),
    };
    match create_if_not_exist(&format!(
        "{}{}",
        src_folder, "\\createExtProjectsSymLinks.bat"
    )) {
        Ok(_) => println!("createExtProjectsSymLinks.bat has created!"),
        Err(e) => println!("createExtProjectsSymLinks.bat: {}", e),
    };

    match create_if_not_exist(&format!(
        "{}\\{}{}",
        build_folder, app_name, "_bar_Int.properties"
    )) {
        Ok(_) => println!("Int.properties has created!"),
        Err(e) => println!("Int.properties: {}", e),
    };

    match create_if_not_exist(&format!("{}{}", build_folder, "\\version.txt")) {
        Ok(_) => println!("version.txt has created!"),
        Err(e) => println!("version.txt: {}", e),
    };

    let f = create_if_not_exist(&format!("{}{}", build_folder, "\\.gitignore"));
    match f {
        Ok(mut _f) => _f.write_fmt(format_args!("{}\n", ".out"))?,
        Err(e) => println!("Gitignore at build: {}", e),
    };

    match create_if_not_exist(&format!("{}\\{}{}", rn_folder, app_name, ".prereq")) {
        Ok(_) => println!("prereq file has created!"),
        Err(e) => println!("prereq file: {}", e),
    };

    call_mqsireadbar(&cfg).expect("Error during reading bar properties");
    let props = parse_properties_file(&cfg).expect("Error whilist parsing props");
    create_rn_iib(&cfg, props).expect("Error while creating RN");

    Ok(())
}

/// Creates a file if it is not already existed
fn create_if_not_exist(p: &str) -> io::Result<std::fs::File> {
    if Path::new(&p).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File already exists",
        ));
    }
    File::create(&p)
}

/// Moves directory throug system call
fn move_dir_through_cmd(from: &str, to: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "move", from, to])
            .output()
            .expect("failed to execute process");
        Ok(())
    } else {
        Err("Not supported OS".to_owned())
    }
}

/// Creates an empty table of width and height, also column names are required
fn create_empty_table(width: usize, height: usize, col_names: Vec<&str>) -> Result<Table, i32> {
    if col_names.len() != width {
        return Err(0);
    }
    if height == 0 {
        return Err(0);
    }
    let mut named_row: Vec<TableCell> = Vec::with_capacity(width);
    for col_name in col_names {
        named_row.push(
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().add_text(col_name).size(22).bold()),
            ),
        )
    }
    let named_row = TableRow::new(named_row);
    let mut rows: Vec<TableRow> = Vec::with_capacity(height);
    rows.push(named_row);
    if height > 1 {
        for _ in 0..height - 1 {
            let mut row: Vec<TableCell> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(TableCell::new().add_paragraph(Paragraph::new()))
            }
            let row = TableRow::new(row);
            rows.push(row);
        }
    }

    Ok(Table::new(rows))
}

fn fill_table(mut tbl: Table, props: (Vec<String>, Vec<String>)) -> Result<Table, ()> {
    for cntr in 1..tbl.rows.len() {
        tbl.rows[cntr].cells[0].children = vec![TableCellContent::Paragraph(
            Paragraph::new().add_run(Run::new().add_text(&props.0[cntr]).size(18)),
        )];
        tbl.rows[cntr].cells[1].children = vec![TableCellContent::Paragraph(
            Paragraph::new().add_run(Run::new().add_text(&props.1[cntr]).size(18)),
        )];
    }

    Ok(tbl)
}

/// From readbar output to vectors of properties (hashmapuse possibly)
fn parse_properties_file(cfg: &Config) -> Result<(Vec<String>, Vec<String>), ()> {
    let mut props_names = Vec::<String>::new();
    let mut props = Vec::<String>::new();

    // Just write file with properties instantly here
    let mut prop_int_file = File::create(format!(
        "{}\\build\\{}{}",
        &cfg.path_to_app, &cfg.app_name, "_bar_Int.properties"
    ))
    .expect("Error while creating file");

    if let Ok(lines) = read_lines(&cfg.path_to_out_props) {
        for line in lines {
            if let Ok(prop) = line {
                if prop.chars().fold(
                    0,
                    |acc, x| if matches!(x, '#' | '=') { acc + 1 } else { acc },
                ) == 2
                {
                    prop_int_file
                        .write_fmt(format_args!("{}\n", &prop))
                        .expect("Error while writing to Int props file");
                    let splitted: Vec<_> = prop.split('=').collect();

                    let mut prop_name = splitted[0].to_owned();
                    prop_name.retain(|c| !matches!(c, ' ' | '\n'));
                    let mut prop_value = splitted[1].to_owned();
                    prop_value.retain(|c| !matches!(c, ' ' | '\n'));

                    props_names.push(prop_name);
                    props.push(prop_value);
                }
            }
        }
    }
    Ok((props_names, props))
}

/// Returns an iterator through lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
