# UX v6 embedded assets

Exact SVG exports from the user's Soft Gallery UX v6 page in Figma file
`U1kWw1K5NZBgEYkuxMz3Qb`, retrieved 18 November 2026. Source navigation:
`199:2198`; Sequences: `199:1694`; Map actions: `199:2197`.

`manifest.json` records SHA-256 and the unmodified SVG root dimensions. Root dimensions
and vector content are retained. Icons render at the source screen's instance slot:
18px navigation icons (Home's 24px master scales into its 18px slot), 15px Activity,
15×14.325px Settings, 10.5×6px expansion chevron, and 16px action icons.
The official logo renders untinted in a 30×30px image slot. Monochrome SVGs use GPUI's
foreground tint for neutral/selected states; the full-color logo is never tinted.

Runtime paths are `v6/<name>.svg`, served by the existing embedded AssetSource.
There is no runtime Figma URL or network dependency. Existing scientific/menu assets
remain available for their existing commands.
