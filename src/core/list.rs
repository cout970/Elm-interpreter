use ast::Type;
use constructors::type_of;

pub fn get_list_types() -> Vec<(&'static str, Type)> {
    get_list_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_list_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("cons",      "a -> List a -> List a"),
        ("::",        "a -> List a -> List a"),
        ("map2",      "(a -> b -> result) -> List a -> List b -> List result"),
        ("map3",      "(a -> b -> c -> result) -> List a -> List b -> List c -> List result"),
        ("map4",      "(a -> b -> c -> d -> result) -> List a -> List b -> List c -> List d -> List result"),
        ("map5",      "(a -> b -> c -> d -> e -> result) -> List a -> List b -> List c -> List d -> List e -> List"),
        ("fromArray", "Array a -> List a"),
        ("toArray",   "List a -> Array a"),
        ("sortBy",    "(a -> comparable) ->  List a -> List a"),
        ("sortWith",  "(a -> a -> Order) ->  List a -> List a"),
    ]
    //@formatter:on
}