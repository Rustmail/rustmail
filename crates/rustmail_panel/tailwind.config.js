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
                    DEFAULT: "rgb(var(--fg) / 0.08)",
                    strong: "rgb(var(--fg) / 0.14)",
                },
                accent: {
                    DEFAULT: token("accent"),
                    hover: token("accent-hover"),
                    fg: token("accent-fg"),
                    subtle: "rgb(var(--accent) / 0.14)",
                },
                blurple: {
                    DEFAULT: token("blurple"),
                    fg: token("blurple-fg"),
                },
                positive: { DEFAULT: token("positive"), subtle: "rgb(var(--positive) / 0.12)" },
                negative: { DEFAULT: token("negative"), subtle: "rgb(var(--negative) / 0.12)" },
                warning: { DEFAULT: token("warning"), subtle: "rgb(var(--warning) / 0.12)" },
            },
            fontFamily: {
                sans: ["Geist", "ui-sans-serif", "system-ui", "sans-serif"],
                mono: ["Geist Mono", "ui-monospace", "SFMono-Regular", "monospace"],
            },
            fontSize: {
                title: ["28px", { lineHeight: "34px", letterSpacing: "-0.01em" }],
                section: ["20px", "28px"],
                figure: ["32px", { lineHeight: "36px", letterSpacing: "-0.01em" }],
                display: ["48px", { lineHeight: "52px", letterSpacing: "-0.02em" }],
            },
            transitionTimingFunction: {
                out: "cubic-bezier(0.22, 1, 0.36, 1)",
            },
        },
    },
    plugins: [],
};
