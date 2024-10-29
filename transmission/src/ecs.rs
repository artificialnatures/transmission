use uuid::Uuid;
use std::collections::HashMap;

type Identifier = Uuid;

struct EntityTable {
    root_entity: Identifier,
    entities: Vec<Identifier>,
    components: HashMap<Identifier, Vec<Identifier>>,
    relations: HashMap<Identifier, Vec<Identifier>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root_entity = Identifier::new_v4();
        let entity_table = EntityTable {
            root_entity: root_entity,
            entities: vec![root_entity],
            components: HashMap::from([(root_entity, vec![])]),
            relations: HashMap::from([(root_entity, vec![])])
        };
        assert_eq!(entity_table.root_entity, root_entity);
        assert_eq!(entity_table.entities[0], root_entity);
        assert_eq!(entity_table.components[&root_entity], vec![]);
        assert_eq!(entity_table.relations[&root_entity], vec![]);
    }
}