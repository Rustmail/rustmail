const token = (name) => `rgb(var(--${name}) / <alpha-value>)`;

module.exports = {
    mode: "jit",
    content: {
        files: ["index.html", "src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                surface: {
                    sunken: token("surface-sunken"),
                    page: token("surface-page"),
                    DEFAULT: token("surface"),
                    raised: token("surface-raised"),
                    hover: token("surface-hover"),
                    inverse: token("surface-inverse"),
                },
                fg: {
                    DEFAULT: token("fg"),
                    secondary: token("fg-secondary"),
                    muted: token("fg-muted"),
                    inverse: token("fg-inverse"),
                },
                line: {
                    DEFAULT: token("line"),
                    strong: token("line-strong"),
                },
                accent: {
                    DEFAULT: token("accent"),
                    hover: token("accent-hover"),
                    fg: token("accent-fg"),
                    subtle: token("accent-subtle"),
                },
                blurple: {
                    DEFAULT: token("blurple"),
                    fg: token("blurple-fg"),
                },
                positive: { DEFAULT: token("positive"), subtle: token("positive-subtle") },
                negative: { DEFAULT: token("negative"), subtle: token("negative-subtle") },
                warning: { DEFAULT: token("warning"), subtle: token("warning-subtle") },
            },
            fontFamily: {
                sans: ["Inter", "ui-sans-serif", "system-ui", "sans-serif"],
                mono: ["JetBrains Mono", "ui-monospace", "SFMono-Regular", "Menlo", "monospace"],
            },
            fontSize: {
                title: ["24px", "32px"],
                section: ["20px", "28px"],
                figure: ["30px", { lineHeight: "38px", letterSpacing: "-0.015em" }],
                display: ["48px", { lineHeight: "52px", letterSpacing: "-0.015em" }],
            },
            borderRadius: {
                none: "0",
                sm: "2px",
                DEFAULT: "4px",
                md: "4px",
                lg: "4px",
                xl: "6px",
                "2xl": "6px",
                "3xl": "6px",
                full: "9999px",
            },
            transitionTimingFunction: {
                out: "cubic-bezier(0.22, 1, 0.36, 1)",
            },
        },
    },
    plugins: [],
};
