//! Row, input and enum references must use the same type allocation.

use super::{fixture, syntax};
use crate::generators::rust::{allocation::Names, models, procedures, repositories};

#[test]
fn colliding_models_keep_consistent_references() {
    let schema = fixture::schema();
    let names = Names::new(&schema);
    let models = models::render(&schema, &names);
    let repositories = repositories::render(&schema, &names);
    let procedures = procedures::render(&schema, &names);
    for source in [&models, &repositories, &procedures] {
        syntax::assert_unique(source);
    }
    let proposal = schema.table("proposal").unwrap();
    assert_eq!(
        names.row(schema.table("new_proposal").unwrap()),
        "NewProposal"
    );
    assert_eq!(names.create(proposal), "NewProposal2");
    assert_eq!(names.patch(proposal), "PatchProposal2");
    assert!(repositories.contains("input: NewProposal2"));
    assert!(procedures.contains("web::Json<NewProposal2>"));
    assert!(procedures.contains("web::Json<PatchProposal2>"));
    let enum_name = names.enum_type("proposal");
    assert_ne!(enum_name, names.row(proposal));
    assert!(models.contains(&format!("pub state: {enum_name}")));
    assert!(models.contains("#[sqlx(type_name = \"proposal\")]"));
}
