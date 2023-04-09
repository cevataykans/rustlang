mod plant_structures {
    pub mod roots {
        static TEST: String = "Hello World".to_string();

        pub mod products {
            pub(in crate::plant_structures::roots) struct Cytokinin {}
        }
        use products::Cytokinin; // ok: in `roots` module
    }

    use plant_structures::roots::products::Cytokinin; // error: `Cytokinin` is private
    pub mod stems {}

    pub mod leaves {}
}
