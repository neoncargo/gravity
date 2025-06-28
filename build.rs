fn main() {
    println!("cargo:rerun-if-changed=src/app/model/universe/physics/collision_logic/bodies_logic.c");

    cc::Build::new()
        .file("src/app/model/universe/physics/collision_logic/bodies_logic.c")
        .compile("bodies_logic");
}
