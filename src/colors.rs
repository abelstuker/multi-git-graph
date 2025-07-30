#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub name: &'static str,
    pub colors: [&'static str; 5],
}

#[allow(dead_code)]
impl ColorScheme {
    pub fn get_color(&self, index: usize) -> &'static str {
        self.colors.get(index).unwrap_or(&self.colors[0])
    }
}

pub const SCHEMES: &[ColorScheme] = &[
    ColorScheme {
        name: "github",
        colors: ["#eeeeee", "#2ea043", "#3fb950", "#50d05d", "#a1f1a8"],
    },
    ColorScheme {
        name: "halloween",
        colors: ["#eeeeee", "#04001b", "#ff9711", "#ffc722", "#fdf156"],
    },
    ColorScheme {
        name: "amber",
        colors: ["#eeeeee", "#ff6f00", "#ffb300", "#ffd54f", "#ffecb3"],
    },
    ColorScheme {
        name: "blue",
        colors: ["#eeeeee", "#0d47a1", "#1e88e5", "#64b5f6", "#bbdefb"],
    },
    ColorScheme {
        name: "bluegrey",
        colors: ["#eeeeee", "#263238", "#546e7a", "#90a4ae", "#cfd8dc"],
    },
    ColorScheme {
        name: "brown",
        colors: ["#eeeeee", "#3e2723", "#6d4c41", "#a1887f", "#d7ccc8"],
    },
    ColorScheme {
        name: "cyan",
        colors: ["#eeeeee", "#006064", "#00acc1", "#4dd0e1", "#b2ebf2"],
    },
    ColorScheme {
        name: "deeporange",
        colors: ["#eeeeee", "#bf360c", "#f4511e", "#ff8a65", "#ffccbc"],
    },
    ColorScheme {
        name: "deeppurple",
        colors: ["#eeeeee", "#311b92", "#5e35b1", "#9575cd", "#d1c4e9"],
    },
    ColorScheme {
        name: "green",
        colors: ["#eeeeee", "#1b5e20", "#43a047", "#81c784", "#c8e6c9"],
    },
    ColorScheme {
        name: "grey",
        colors: ["#eeeeee", "#212121", "#616161", "#9e9e9e", "#e0e0e0"],
    },
    ColorScheme {
        name: "indigo",
        colors: ["#eeeeee", "#1a237e", "#3949ab", "#7986cb", "#c5cae9"],
    },
    ColorScheme {
        name: "lightblue",
        colors: ["#eeeeee", "#01579b", "#039be5", "#4fc3f7", "#b3e5fc"],
    },
    ColorScheme {
        name: "lightgreen",
        colors: ["#eeeeee", "#33691e", "#7cb342", "#aed581", "#dcedc8"],
    },
    ColorScheme {
        name: "lime",
        colors: ["#eeeeee", "#827717", "#c0ca33", "#dce775", "#f0f4c3"],
    },
    ColorScheme {
        name: "orange",
        colors: ["#eeeeee", "#e65100", "#fb8c00", "#ffb74d", "#ffe0b2"],
    },
    ColorScheme {
        name: "pink",
        colors: ["#eeeeee", "#880e4f", "#e91e63", "#f06292", "#f8bbd0"],
    },
    ColorScheme {
        name: "purple",
        colors: ["#eeeeee", "#4a148c", "#8e24aa", "#ba68c8", "#e1bee7"],
    },
    ColorScheme {
        name: "red",
        colors: ["#eeeeee", "#b71c1c", "#e53935", "#e57373", "#ffcdd2"],
    },
    ColorScheme {
        name: "teal",
        colors: ["#eeeeee", "#004d40", "#00897b", "#4db6ac", "#b2dfdb"],
    },
    ColorScheme {
        name: "yellow",
        colors: ["#eeeeee", "#f57f17", "#ffd835", "#fff176", "#fff9c4"],
    },
    ColorScheme {
        name: "moon",
        colors: ["#eeeeee", "#4f2266", "#48009a", "#00a1f3", "#6bcdff"],
    },
    ColorScheme {
        name: "psychedelic",
        colors: ["#eeeeee", "#ff00ab", "#fa3fbc", "#fb6dcc", "#faafe1"],
    },
];

impl ColorScheme {
    pub fn find_by_name(name: &str) -> Option<&'static ColorScheme> {
        SCHEMES.iter().find(|scheme| scheme.name == name)
    }

    #[allow(dead_code)]
    pub fn default() -> &'static ColorScheme {
        &SCHEMES[0] // GitHub colors as default
    }
}
