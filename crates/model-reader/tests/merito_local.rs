use driftlens_model_reader::source::java::parse_directory;
use std::path::Path;

#[test]
#[ignore = "local only — requires Merito source"]
fn test_parse_merito_entities() {
    let dir = Path::new("/Users/pedrocampos/Documents/Projetos/Ativos/Merito/api/src/main/java/br/com/sistemamerito/models");
    let entities = parse_directory(dir).expect("Failed to parse directory");

    println!("Entities found: {}", entities.len());
    for entity in &entities {
        println!("\n{} → {}", entity.entity_name, entity.table_name);
        for col in &entity.columns {
            println!(
                "  {} ({:?}) nullable:{}",
                col.column_name, col.length, col.nullable
            );
        }
    }

    assert!(!entities.is_empty());
}
