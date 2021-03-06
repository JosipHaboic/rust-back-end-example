pub mod base {
    /// Layer Supertype - Marker Trait
    /// A type that acts as the supertype for all types in its layer.
    pub trait Entity {}

    /// Gateway
    /// An object that encapsulates access to an external system or resource
    pub trait Gateway<'a> {
        type Connection;

        fn init(connection: &'a Self::Connection) -> Self;
    }
}

pub mod domain_logic {
    /// Transaction Script - Command Pattern Form
    /// Organizes business logic by procedures where each procedure handles a
    /// single request from the presentation.
    pub trait TransactionScript {
        type Output;
        type Connection;
        type Params;
        fn execute(
            self: &Self,
            connection: &Self::Connection,
            params: &Self::Params,
        ) -> Self::Output;
    }
}

pub mod object_relational {
    pub mod structural {
        /// Identity Field
        /// Saves a database ID field in an object to maintain identity between
        /// an in-memory object and a database row.
        pub trait IdentityField {
            type IdType;

            fn id(self: &Self) -> &Self::IdType;
        }
    }
}

pub mod data_source {
    use super::base::Gateway;

    /// Table Gateway
    /// An object that acts as a Gateway (466) to a database table.
    /// One instance handles all the rows in the table.
    pub trait TableGateway<'a>: Gateway<'a> {
        type Model;
        type Params;
        type Error;

        // CRUD operations
        fn insert(self: &Self, params: &Self::Params) -> Result<(), Self::Error>;
        // find by id or return "all" if id is not provided
        fn find(
            self: &Self,
            id: Option<&str>,
        ) -> Result<Vec<Self::Model>, Self::Error>;
        fn update(self: &Self, params: &Self::Params) -> Result<(), Self::Error>;
        fn delete(self: &Self, id: &str) -> Result<(), Self::Error>;
    }
}
