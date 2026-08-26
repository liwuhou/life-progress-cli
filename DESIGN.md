---
name: Life Progress CLI
description: A precise terminal instrument for measuring life in days.
colors:
  primary: "#67e8f9"
  accent: "#facc15"
  success: "#4ade80"
  error: "#f87171"
  muted: "#94a3b8"
  text: "#e2e8f0"
typography:
  body:
    fontFamily: "monospace"
    fontSize: "1em"
    fontWeight: 400
    lineHeight: 1.4
  label:
    fontFamily: "monospace"
    fontSize: "1em"
    fontWeight: 700
    lineHeight: 1.4
rounded: {}
spacing:
  row: "1 line"
  section: "2 lines"
components: {}
---

# Design System: Life Progress CLI

## Overview

**Creative North Star: "The Minimal Measuring Instrument"**

The terminal is treated as a precise measuring instrument, not a decorative dashboard. Strong alignment, restrained color, and one clear hierarchy turn personal statistics into a calm ritual that can be read at a glance.

Progress output is a compact, single-screen report. Search and view output share the same disciplined rows, with color reserved for state and emphasis. Unicode is intentional: the progress bar is a measurement scale, never an icon or ornament.

**Key Characteristics:**
- Monospaced, aligned, instrument-like readouts.
- One-screen progress summary with clear hierarchy.
- ANSI color and Unicode enabled for modern interactive terminals.
- Search results use consistent rows rather than ad-hoc inline records.

## Colors

Use a near-neutral terminal canvas and reserve color for semantics: cyan for the instrument title, yellow for the measured percentage, green for completed progress, red for errors, and muted text for secondary context.

### Primary
- **Instrument Cyan** (#67e8f9): Title and primary section markers.

### Tertiary
- **Measurement Yellow** (#facc15): The percentage result and key numeric emphasis.
- **Completion Green** (#4ade80): Filled progress-bar segments.
- **Error Red** (#f87171): No-match and invalid-state messaging.

### Neutral
- **Readable Text** (#e2e8f0): Main values and labels.
- **Quiet Slate** (#94a3b8): Dividers and supporting context.

### Named Rules
**The Signal-Only Color Rule.** Color must communicate hierarchy or state; never color an entire block without a reason.

## Typography

**Display Font:** Terminal monospace
**Body Font:** Terminal monospace
**Label/Mono Font:** Terminal monospace

**Character:** Fixed-width typography makes alignment measurable. Weight, case, and spacing create hierarchy without introducing ornamental type.

### Hierarchy
- **Display** (bold, terminal size): The `LIFE PROGRESS` title only.
- **Title** (bold, terminal size): Section titles such as `NATION SEARCH`.
- **Body** (regular, terminal size): Values and explanatory output.
- **Label** (bold, terminal size): Fixed-width field names and result headings.

## Layout

Progress uses a compact report with two-space outer indentation and a fixed label column. The title, divider, fields, measurement bar, and recovery hint are separated into readable sections without requiring scrolling. Search and view preserve the same left edge and use one result per logical block.

## Elevation & Depth

The interface is flat by design. Dividers, whitespace, and alignment establish structure; no shadows, boxes, or simulated panels are used.

## Shapes

There are no rounded containers or decorative borders. A thin text divider and block-based measurement bar are the only repeated geometry.

## Components

### Progress Readout
- **Character:** A single-screen measurement report.
- **Structure:** Title, divider, aligned fields, Unicode bar, and a quiet closing prompt.
- **States:** Positive progress uses green fill and yellow percentage; missing profile uses a clear recovery message.

### Nation Result
- **Character:** A ranked, scannable data row.
- **Structure:** Highlighted nation name followed by aligned life-expectancy values on the next line.
- **States:** Fuzzy matches highlight the matched characters; no matches use the error color and an actionable hint.

### Display Preferences
- **Storage:** Optional `~/.config/life_progress/cli.toml`, separate from the shared profile.
- **Report fields:** `show_born`, `show_nation`, `show_gender`, `show_elapsed`, `show_remaining`, `show_complete`, and `show_timeline` default to visible and can be disabled independently.
- **Text mode:** `mode = "text"` renders `text_template` without report chrome. Supported placeholders are `{born}`, `{nation}`, `{gender}`, `{elapsed}`, `{remaining}`, `{complete}`, and `{timeline}`.

## Do's and Don'ts

### Do:
- **Do** align labels and numeric values to a stable terminal column.
- **Do** keep one-screen progress output compact and complete.
- **Do** use Unicode blocks only for the measurement bar.
- **Do** keep redirected output readable even when ANSI color is unavailable.

### Don't:
- **Don't** add cards, boxes, emoji, or decorative gradients.
- **Don't** use color as decoration or make every value equally loud.
- **Don't** replace precise labels with vague motivational copy.
