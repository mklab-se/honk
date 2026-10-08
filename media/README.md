# honk artwork

Generated with [ailloy](https://github.com/mklab-se/ailloy) (`ailloy image`), node
`microsoft-foundry/MAI-Image-2.6-Flash`, in the house style of the other MKLab tools
(monochrome graphite and ink, cross-hatching, white background). Each image was picked from
several candidates by eye.

## honk-horizontal.png (1536x1024)

Reference image: `cosq/media/cosq-horizontal.png` (style only). Command:
`ailloy image "<prompt>" --ref cosq-horizontal.png --size 1536x1024 --variants 3`

Prompt:

> Monochrome graphite pencil and ink sketch, detailed cross-hatching, pure white background that fades out softly at the edges, no colour, no title text, whimsical technical illustration in the style of a vintage engraving. A cheerful developer in a baseball cap stands beside a vintage open-top Model T era car parked on a cobbled street, squeezing a big brass bulb horn mounted on the car. Bold hand-drawn sound lines and the words HONK HONK burst from the horn bell in comic lettering. On the car seat sits an open laptop whose screen shows "$ honk" and "Honk, honk!". Background: a sketched old-town skyline with a church spire and a few birds startled into the sky. Wide 3:2 composition, subject on the left half, horn blast across the right half.

## honk-vertical.png (1024x1536)

Reference image: `honk-horizontal.png`. Command:
`ailloy image "<prompt>" --ref honk-horizontal.png --size 1024x1536 --variants 2`

Prompt:

> Same scene, same style and same character, recomposed as a tall portrait: the vintage car and the developer squeezing the big brass bulb horn in the lower half, the open laptop on the car seat showing "$ honk" and "Honk, honk!", the HONK HONK lettering, sound lines and startled birds rising into the upper half, old-town street with a church spire behind. Monochrome graphite and ink, detailed cross-hatching, pure white background fading out at the edges, no colour.

## car-lineart.png (1536x1024)

The source for the terminal's Braille car (`crates/honk/assets/car-braille.txt`, made by
`scripts/car_to_braille.py`). A generic 1920s roadster, no brand. Command:
`ailloy image "<prompt>" --size 1536x1024 --variants 2`

Prompt:

> Clean black line drawing on a pure white background, a vintage 1920s open-top roadster car seen from the side at a slight three-quarter angle, facing right, spoked wheels, running boards, folded soft top, a brass bulb horn mounted by the windscreen. Bold uniform outlines, minimal shading, no text, no logo, no background, no shadow, the car fills the frame.
