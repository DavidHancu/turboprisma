use relative_path::RelativePathBuf;

pub fn obtain_schema_path(schema_argument: &Option<&str>) -> RelativePathBuf {
    RelativePathBuf::new().join(match schema_argument {
        Some(value) => value,
        None => "./prisma/schema.prisma",
    })
}

#[allow(dead_code)]
pub fn obtain_schema_paths(schema_argument: &Option<Vec<&str>>) -> Vec<RelativePathBuf> {
    match schema_argument {
        Some(values) => values
            .iter()
            .map(|path| obtain_schema_path(&Some(path)))
            .collect(),
        None => vec![obtain_schema_path(&None)],
    }
}

pub struct SchemaWithArgument {
    pub schema_path: RelativePathBuf,
    pub schema_argument: Option<String>,
}

pub fn obtain_schema_path_with_argument(
    schema_argument: &Option<&str>,
) -> SchemaWithArgument {
    SchemaWithArgument {
        schema_path: RelativePathBuf::new().join(match schema_argument {
            Some(value) => value,
            None => "./prisma/schema.prisma",
        }),
        schema_argument: match schema_argument {
            Some(value) => Some(value.to_string()),
            None => None,
        },
    }
}

#[allow(dead_code)]
pub fn obtain_schema_paths_with_argument(
    schema_argument: &Option<Vec<&str>>,
) -> Vec<SchemaWithArgument> {
    match schema_argument {
        Some(values) => values
            .iter()
            .map(|path| obtain_schema_path_with_argument(&Some(path)))
            .collect(),
        None => vec![obtain_schema_path_with_argument(&None)],
    }
}
