#!/usr/bin/env python3
"""Build the deterministic TrueType fixture used by differential tests."""

from pathlib import Path

from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib.tables._c_m_a_p import CmapSubtable


HERE = Path(__file__).resolve().parent
OUTPUT = HERE / "reference.ttf"
GLYPH_ORDER = [".notdef", "space", "triangle", "acute", "Aacute", "nested"]


def empty_glyph():
    return TTGlyphPen(None).glyph()


def polygon(points):
    pen = TTGlyphPen(None)
    pen.moveTo(points[0])
    for point in points[1:]:
        pen.lineTo(point)
    pen.closePath()
    return pen.glyph()


def composite(components):
    pen = TTGlyphPen(set(GLYPH_ORDER))
    for glyph_name, transform in components:
        pen.addComponent(glyph_name, transform)
    return pen.glyph()


def build_font():
    glyphs = {
        ".notdef": polygon([(40, 0), (40, 700), (560, 700), (560, 0)]),
        "space": empty_glyph(),
        "triangle": polygon([(50, 0), (300, 700), (550, 0)]),
        "acute": polygon([(240, 760), (330, 900), (400, 900), (300, 760)]),
        "Aacute": composite(
            [
                ("triangle", (1, 0, 0, 1, 0, 0)),
                ("acute", (1, 0, 0, 1, 0, 0)),
            ]
        ),
        "nested": composite(
            [
                ("Aacute", (1, 0, 0, 1, 0, 0)),
                ("triangle", (0.5, 0, 0, 0.5, 300, 0)),
            ]
        ),
    }

    builder = FontBuilder(1000, isTTF=True)
    builder.setupGlyphOrder(GLYPH_ORDER)
    builder.setupCharacterMap(
        {0x20: "space", 0x41: "triangle", 0x0301: "acute", 0x00C1: "Aacute", 0x1F600: "nested"}
    )
    builder.setupGlyf(glyphs, calcGlyphBounds=True)
    builder.setupHorizontalMetrics({name: (600, 0) for name in GLYPH_ORDER})
    builder.setupHorizontalHeader(ascent=950, descent=-250)
    builder.setupNameTable(
        {
            "familyName": "Glyphweave Differential",
            "styleName": "Regular",
            "uniqueFontIdentifier": "GlyphweaveDifferential-Regular-1",
            "fullName": "Glyphweave Differential Regular",
            "psName": "GlyphweaveDifferential-Regular",
        }
    )
    builder.setupOS2(
        sTypoAscender=950,
        sTypoDescender=-250,
        usWinAscent=950,
        usWinDescent=250,
    )
    builder.setupPost()
    builder.setupMaxp()

    variation = CmapSubtable.newSubtable(14)
    variation.platformID = 0
    variation.platEncID = 5
    variation.language = 0
    variation.cmap = {}
    variation.uvsDict = {0xFE0F: [(0x41, None), (0x42, "Aacute")]}
    builder.font["cmap"].tables.append(variation)
    builder.font["head"].created = 0
    builder.font["head"].modified = 0
    builder.font.recalcTimestamp = False
    builder.font.save(OUTPUT, reorderTables=True)


if __name__ == "__main__":
    build_font()
