pub mod base {
    /// Layer Supertype - Marker Trait
    /// A type that acts as the supertype for all types in its layer.
    pub trait Entity {}

    /// Gateway
    /// An object that encapsulates access to an external system or resource
    pub trait Gateway {
        type Connection;

        fn init(connection: Self::Connection) -> Self;
    }
}

pub mod domain_logic {
    /// Transaction Script - Command Pattern Form
    /// Organizes business logic by procedures where each procedure handles a
    /// single request from the presentation.
    pub trait TransactionScript {
        fn execute(self: &Self) -> bool;
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
    pub trait TableGateway: Gateway {
        type Model;
        type Params;

        fn create_table(self: &Self) -> bool;
        fn drop_table(self: &Self) -> bool;
        // CRUD operations
        fn insert(self: &Self, params: &Self::Params) -> bool;
        fn find(self: &Self, id: &str) -> Option<Self::Model>;
        fn update(self: &Self, params: &Self::Params) -> bool;
        fn delete(self: &Self, id: &str) -> bool;
    }
}
