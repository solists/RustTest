pub mod obj_model{
    use std::collections::HashMap;
    use std::path::Path;
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use std::string::String;
    use crate::geometry::point::Point3;
    use crate::geometry::vector::Vector3;

    pub struct Model {
        pub vertices: HashMap<u32, Point3<f32>>,
        pub faces: Vec<Face>,
        pub normals: HashMap<u32, Vector3>,
    }


    impl Model {
        // Returns Point with abs max values of vertices coordinates,
        // independently {x, y, z}
        // Used for scaling mostly
        pub fn max_coord(&self) -> Point3<f32> {
            let mut max_p = Point3{x:0., y: 0., z: 0.};
            for (_, p) in &self.vertices {
                if p.x.abs() > max_p.x {
                    max_p.x = p.x.abs();
                }
                if p.y.abs() > max_p.y {
                    max_p.y = p.y.abs();
                }
                if p.z.abs() > max_p.z {
                    max_p.z = p.z.abs();
                }
            }
            max_p
        }
    }

    // Face consists of 3 props, so props describe one vertex:
    // Coordinates, texture and normal, props is actually vertex properties
    pub struct Props {
        pub v: u32,
        pub vt: u32,
        pub vn: u32,
    }

    pub struct Face {
        pub f1: Props,
        pub f2: Props,
        pub f3: Props,
    }

    impl Model {
        pub fn new() -> Model {
            Model{vertices: HashMap::new(), faces: Vec::new(), normals: HashMap::new()}
        }
        // Read object from .obj file, not supporting textures for now
        pub fn read_obj(&mut self, filename: &str) -> Option<Model> {
            let path = Path::new(filename);
            let f = match File::open(path)  {
                Err(_) => return None,
                Ok(f) => f
            };
            let mut br = BufReader::new(f);

            let mut vertex_counter: u32 = 1;
            let mut normals_counter: u32 = 1;
            // Read file line by line to the end
            loop{
                let mut line= String::new();
                let splitted_line: Vec<&str> = match br.read_line(&mut line) {
                    Err(_) => break,
                    // If EOF
                    Ok(0) => break,
                    Ok(_) => line.split_whitespace().collect::<Vec<&str>>()
                };
                // Empty lines just skipped
                if splitted_line.len() == 0 {continue;}
                // Vertex property, consists of 3 coordinates
                if splitted_line[0] == "v" {
                    if splitted_line.len() != 4 { return None; }
                    else {
                        let x: f32 = splitted_line[1].trim().parse().expect("Error, while parsing a vertex");
                        let y: f32 = splitted_line[2].trim().parse().expect("Error, while parsing a vertex");
                        let z: f32 = splitted_line[3].trim().parse().expect("Error, while parsing a vertex");
                        self.vertices.insert(vertex_counter, Point3{x: x, y: y, z: z});
                        vertex_counter += 1;
                    }
                }
                // Face property, consists of 3 bended vertices indexes & textures & normals
                else if splitted_line[0] == "f" {
                    if splitted_line.len() < 4 { return None; }
                    else{
                        // Distinguish vertices indexes from other props
                        let v1 = splitted_line[1].split('/').collect::<Vec<&str>>();
                        let v2 = splitted_line[2].split('/').collect::<Vec<&str>>();
                        let v3 = splitted_line[3].split('/').collect::<Vec<&str>>();

                        // Returns a props structure from v/vt/vn string
                        let parse_face_to_props = |vertex: Vec<&str>| {
                            Props {
                                v: vertex[0].trim().parse().expect("Error"),
                                // Often vn is just skipped (like//), match it manually
                                vt: match vertex[1].trim().parse() {
                                    Ok(value) => value,
                                    Err(_) => 0,
                                },
                                vn: vertex[2].trim().parse().expect("Error")
                            }
                        };

                        let props1 = parse_face_to_props(v1);
                        let props2 = parse_face_to_props(v2);
                        let props3 = parse_face_to_props(v3);

                        let faces = Face{f1: props1, f2: props2, f3: props3};

                        self.faces.push(faces);
                    }
                }
                else if splitted_line[0] == "vn" {
                    if splitted_line.len() != 4 { return None; }
                    else {
                        let x: f32 = splitted_line[1].trim().parse().expect("Error, while parsing a vertex");
                        let y: f32 = splitted_line[2].trim().parse().expect("Error, while parsing a vertex");
                        let z: f32 = splitted_line[3].trim().parse().expect("Error, while parsing a vertex");
                        self.normals.insert(normals_counter, Vector3{x: x, y: y, z: z});
                        normals_counter += 1;
                    }
                }
                else {continue;}
            }
            None
        }
    }
}