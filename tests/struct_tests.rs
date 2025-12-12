mod doc_display {
    mod struct_tests {
        use {
            macroni_n_cheese::DocDisplay,
            std::{f64, fmt::Display, rc::Rc, sync::Arc},
        };

        #[test]
        fn with_interp() {
            /// Value: {value}
            #[derive(DocDisplay)]
            struct Simple {
                value: i32,
            }

            let s = Simple { value: 42 };

            assert_eq!(s.to_string(), "Value: 42")
        }

        #[test]
        fn without_interp() {
            /// A constant message
            #[derive(DocDisplay)]
            struct NoInterpolation;

            let s = NoInterpolation;
            assert_eq!(s.to_string(), "A constant message");
        }

        #[test]
        fn multi_field() {
            /// ({x}, {y}, {z})
            #[derive(DocDisplay)]
            struct Point3D {
                x: i32,
                y: i32,
                z: i32,
            }

            let p = Point3D { x: 1, y: 2, z: 3 };
            assert_eq!(p.to_string(), "(1, 2, 3)");
        }

        #[test]
        fn multiline_doc() {
            /// First line
            /// Second line
            #[derive(DocDisplay)]
            struct MultiLine;

            let m = MultiLine;
            assert_eq!(m.to_string(), "First line\nSecond line");
        }

        #[test]
        fn repeated_field() {
            /// {x} + {x} = {y}
            #[derive(DocDisplay)]
            struct Math {
                x: i32,
                y: i32,
            }

            let m = Math { x: 5, y: 10 };
            assert_eq!(m.to_string(), "5 + 5 = 10");
        }

        #[test]
        fn str_field() {
            /// Hello, {name}!
            #[derive(DocDisplay)]
            struct Greeting {
                name: String,
            }

            let g = Greeting {
                name: "World".to_string(),
            };
            assert_eq!(g.to_string(), "Hello, World!");
        }

        #[test]
        fn nested_structs() {
            /// Inner: {value}
            #[derive(DocDisplay)]
            struct Inner {
                value: i32,
            }

            /// Outer contains: {inner}
            #[derive(DocDisplay)]
            struct Outer {
                inner: Inner,
            }

            let o = Outer {
                inner: Inner { value: 42 },
            };

            assert_eq!(o.to_string(), "Outer contains: Inner: 42");
        }

        #[test]
        fn bool_field() {
            /// Active: {is_active}
            #[derive(DocDisplay)]
            struct Status {
                is_active: bool,
            }

            let active = Status { is_active: true };
            let inactive = Status { is_active: false };

            assert_eq!(active.to_string(), "Active: true");
            assert_eq!(inactive.to_string(), "Active: false");
        }

        #[test]
        fn float_field() {
            /// Pi is approximately {value}
            #[derive(DocDisplay)]
            struct Pi {
                value: f64,
            }

            let pi = Pi {
                value: f64::consts::PI,
            };

            assert_eq!(
                pi.to_string(),
                format!("Pi is approximately {}", f64::consts::PI)
            );
        }

        #[test]
        fn char_field() {
            /// Character: {ch}
            #[derive(DocDisplay)]
            struct CharWrapper {
                ch: char,
            }

            let c = CharWrapper { ch: 'A' };
            assert_eq!(c.to_string(), "Character: A");
        }

        #[test]
        fn unit_struct() {
            /// This is a unit struct
            #[derive(DocDisplay)]
            struct Unit;

            let u = Unit;
            assert_eq!(u.to_string(), "This is a unit struct");
        }

        #[test]
        fn special_chars() {
            /// Special chars: !@#$%^&*()
            #[derive(DocDisplay)]
            struct Special;

            let s = Special;
            assert_eq!(s.to_string(), "Special chars: !@#$%^&*()");
        }

        #[test]
        fn escaped_braces() {
            /// Use {{double braces}} for literal braces
            #[derive(DocDisplay)]
            struct Braces;

            let b = Braces;
            assert_eq!(b.to_string(), "Use {double braces} for literal braces");
        }

        #[test]
        fn ref_field() {
            /// Name: {name}
            #[derive(DocDisplay)]
            struct NameRef<'a> {
                name: &'a str,
            }

            let n = NameRef { name: "Woah" };
            assert_eq!(n.to_string(), "Name: Woah");
        }

        #[test]
        fn box_field() {
            /// Boxed value: {value}
            #[derive(DocDisplay)]
            struct Boxed {
                value: Box<i32>,
            }

            let b = Boxed {
                value: Box::new(42),
            };

            assert_eq!(b.to_string(), "Boxed value: 42");
        }

        #[test]
        fn rc_field() {
            /// (Rc) Shared value: {value}
            #[derive(DocDisplay)]
            struct Shared {
                value: Rc<i32>,
            }

            let s = Shared {
                value: Rc::new(100),
            };

            assert_eq!(s.to_string(), "(Rc) Shared value: 100");
        }

        #[test]
        fn arc_field() {
            /// (Arc) Shared value: {value}
            #[derive(DocDisplay)]
            struct Shared {
                value: Arc<i32>,
            }

            let s = Shared {
                value: Arc::new(100),
            };

            assert_eq!(s.to_string(), "(Arc) Shared value: 100");
        }

        #[test]
        fn custom_type() {
            /// Custom: {custom}
            #[derive(DocDisplay)]
            struct Wrapper {
                custom: CustomDisplay,
            }

            struct CustomDisplay(i32);

            impl Display for CustomDisplay {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "Custom({})", self.0)
                }
            }

            let w = Wrapper {
                custom: CustomDisplay(42),
            };
            assert_eq!(w.to_string(), "Custom: Custom(42)");
        }
    }
}
