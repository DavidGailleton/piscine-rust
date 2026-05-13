const fn color_name<'a>(color: &[u8; 3]) -> &'a str {
    match (color[0], color[1], color[2]) {
        (0, 0, 0) => "pure black",
        (255, 255, 255) => "pure white",
        (255, 0, 0) => "pure red",
        (0, 255, 0) => "pure green",
        (0, 0, 255) => "pure blue",
        (128, 128, 128) => "perfect grey",
        (..=30, ..=30, ..=30) => "almost dark",
        (128.., ..=127, ..=127) => "redish",
        (..=127, 128.., ..=127) => "greenish",
        (..=127, ..=127, 128..) => "blueish",
        (_, _, _) => "unknown",
    }
}

#[cfg(test)]
#[test]
fn test_lifetimes() {
    let name_of_the_best_color;

    {
        let the_best_color = [42, 42, 42];
        name_of_the_best_color = color_name(&the_best_color);
    }

    assert_eq!(name_of_the_best_color, "unknown");
}

#[cfg(test)]
#[test]
fn some_tests() {
    let mut colors: [u8; 3] = [0, 0, 0];

    assert_eq!(color_name(&colors), "pure black");
    colors = [255, 255, 255];
    assert_eq!(color_name(&colors), "pure white");
    colors = [255, 0, 0];
    assert_eq!(color_name(&colors), "pure red");
    colors = [128, 128, 128];
    assert_eq!(color_name(&colors), "perfect grey");
    colors = [30, 30, 30];
    assert_eq!(color_name(&colors), "almost dark");
    colors = [130, 30, 30];
    assert_eq!(color_name(&colors), "redish");
    colors = [120, 30, 30];
    assert_eq!(color_name(&colors), "unknown");
}
