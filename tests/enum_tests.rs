mod doc_display {
    mod enum_tests {
        use macroni_n_cheese::DocDisplay;

        #[test]
        fn enum_unit_variants() {
            #[derive(DocDisplay)]
            enum Color {
                /// Red
                Red,
                /// Green
                Green,
                /// Blue
                Blue,
            }

            assert!(Color::Red.to_string() == "Red");
            assert!(Color::Green.to_string() == "Green");
            assert!(Color::Blue.to_string() == "Blue");
        }

        #[test]
        fn enum_type_level_doc() {
            /// Status
            #[derive(DocDisplay, PartialEq, Debug)]
            enum Status {
                Active,
                Inactive,
            }

            assert!(Status::Active.to_string() == "Status");
            assert!(Status::Inactive.to_string() == "Status");
        }

        #[test]
        fn enum_tuple_variants() {
            /// Error
            #[derive(DocDisplay)]
            enum Error {
                /// Not found: {msg}
                NotFound { msg: String },
                /// Invalid code: {code}
                Invalid { code: i32 },
            }

            let inv = Error::Invalid { code: 404 };
            let nf = Error::NotFound {
                msg: "file.txt".to_string(),
            };

            assert!(nf.to_string() == "Not found: file.txt");
            assert!(inv.to_string() == "Invalid code: 404");
        }

        #[test]
        fn enum_mixed_variants() {
            #[derive(DocDisplay)]
            enum Mixed {
                /// Unit
                Unit,

                /// Tuple variant
                Tuple(i32, i32),

                /// Coordinates: ({x}, {y})
                Struct { x: i32, y: i32 },
            }

            assert!(Mixed::Unit.to_string() == "Unit");
            assert!(Mixed::Tuple(1, 2).to_string() == "Tuple variant");
            assert!(Mixed::Struct { x: 3, y: 4 }.to_string() == "Coordinates: (3, 4)");
        }
    }
}
