Music Scores Design Specification
==================================

Overview
--------

This document describes the design and implementation of the music score rendering feature for Peta, a static site generator. The feature allows users to write music notation in RST files using ABC notation and renders them as interactive music scores in HTML pages.

RST Directive Syntax
--------------------

Music scores are written using the ``musicscore`` directive:

.. code-block:: rst

   .. musicscore:: abc
      :title: Twinkle Twinkle Little Star
      
      X:1
      T:Twinkle Twinkle
      M:4/4
      L:1/4
      K:C
      C C G G | A A G2 |

Supported Options
~~~~~~~~~~~~~~~~~

- ``title``: Display title above the music score

Supported Music Notation Formats
--------------------------------

Primary: ABC Notation
~~~~~~~~~~~~~~~~~~~~~

ABC notation is a text-based music notation system that is human-readable and widely used. It's ideal for this implementation because:

- Compact and easy to write
- Well-established standard
- Has good Rust parser libraries available
- Can be rendered to SVG server-side

Future extensions may include:

- LilyPond
- MusicXML

Architecture
------------

The music score rendering pipeline follows the same pattern as the diagram rendering pipeline:

.. code-block:: text

   RST Content File
       ↓
   RstParser::parse()
       ↓
   process_directives()
       ↓
   MusicScoreHandler (DirectiveHandler)
       ↓
   MusicScoreRenderer::render()
       ↓
   AbcRenderer::render()
       ↓
   calculate_layout() → generate_svg()
       ↓
   HTML output with embedded SVG
       ↓
   CSS/JS Assets (generated from Rust)

File Structure
--------------

.. code-block:: text

   peta/src/content/rst/music_scores/
   ├── mod.rs                    # Module exports + MusicScoreRenderer dispatcher
   ├── models.rs                 # ScoreType enum + ScoreModel
   ├── parser.rs                 # ABC text parser → ScoreModel
   └── renderer.rs               # AbcRenderer with calculate_layout() + generate_svg()

   peta/src/assets/
   ├── css_generator.rs          # MusicScoreCssGenerator
   └── js_generator.rs           # MusicScoreJsGenerator

Files to Modify
----------------

**peta/src/content/rst/mod.rs**

Add module declaration and exports:

.. code-block:: rust

   pub mod music_scores;
   pub use music_scores::*;

**peta/src/content/rst/directives.rs**

Add MusicScoreHandler:

.. code-block:: rust

   pub struct MusicScoreHandler {
       renderer: crate::content::rst::music_scores::MusicScoreRenderer,
   }

   impl MusicScoreHandler {
       pub fn new() -> Result<Self> {
           Ok(Self {
               renderer: crate::content::rst::music_scores::MusicScoreRenderer::new()?,
           })
       }
   }

   impl DirectiveHandler for MusicScoreHandler {
       fn handle(&mut self, score_type: &str, content: &str, options: &HashMap<String, String>) -> Result<String> {
           let content = content.replace("<p>", "").replace("</p>", "\n");
           let title = options.get("title").map(|t| t.as_str());
           self.renderer.render(score_type, &content, title)
       }
   }

**peta/src/content/rst/parser.rs**

Register directive in RstParser::new():

.. code-block:: rust

   directive_handlers.insert(
       "musicscore".to_string(),
       Box::new(crate::content::rst::directives::MusicScoreHandler::new()
           .map_err(|e| Error::Content(format!("Failed to create MusicScoreHandler: {}", e)))?),
   );

**peta/src/assets/css_generator.rs**

Add MusicScoreCssGenerator following DiagramCssGenerator pattern.

**peta/src/assets/js_generator.rs**

Add MusicScoreJsGenerator following DiagramJsGenerator pattern.

**peta/src/assets/pipeline.rs**

Add generate_music_score_assets() method and call it in process_assets().

**peta/src/assets/mod.rs**

Export the new generators:

.. code-block:: rust

   pub use css_generator::{..., MusicScoreCssGenerator};
   pub use js_generator::{..., MusicScoreJsGenerator};

**themes/default/templates/base.html**

Add CSS and JS includes:

.. code-block:: html

   <!-- Music Score Styles (generated from Rust) -->
   <link rel="stylesheet" href="/assets/css/music-scores.css">

   <!-- Music Score Scripts (generated from Rust) -->
   <script src="/assets/js/music-scores.js"></script>

Asset Output Location
---------------------

Following the diagram pattern:

- CSS: ``_out/dist/css/music-scores.css``
- JS: ``_out/dist/js/music-scores.js``

Note: The pipeline writes to ``output_dir/css/`` and ``output_dir/js/``, but the template references them as ``/assets/css/...``.

HTML Output Structure
---------------------

.. code-block:: html

   <div class="music-score-container" data-score-id="..." data-score-type="abc">
     <button class="music-score-download" data-score-id="..." data-score-type="abc" aria-label="Download music score as SVG">
       <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
         <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
         <polyline points="7 10 12 15 17 10"/>
         <line x1="12" y1="15" x2="12" y2="3"/>
       </svg>
     </button>
     <svg viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg" class="music-score-svg">
       <!-- SVG content generated from ABC notation -->
     </svg>
   </div>

ABC Parsing
-----------

The ABC parser extracts:

- Header fields (X, T, M, L, K, etc.)
- Notes and rhythms
- Bar lines and repeats
- Multi-voice support (if needed)

SVG Generation
--------------

Convert parsed ABC to SVG with:

- Staff lines (5 lines per staff)
- Clefs (treble, bass)
- Key signatures
- Time signatures
- Notes with proper placement
- Bar lines
- Title display

CSS Styling
-----------

Features:

- Score container with border and background
- SVG responsive sizing
- Download button styling (matching diagram button)
- Dark mode support

JavaScript Functionality
------------------------

- Download as SVG with copyright (following diagram pattern)
- Optional: Audio playback using Web Audio API
- Optional: Zoom/pan controls

Implementation Checklist
-----------------------

1. Create music_scores module structure
2. Implement ABC parser
3. Implement ABC to SVG renderer
4. Create MusicScoreHandler
5. Register directive in parser
6. Create MusicScoreCssGenerator
7. Create MusicScoreJsGenerator
8. Add asset generation to pipeline
9. Update template with CSS/JS includes
10. Test with various ABC examples

Example Usage
-------------

Simple melody:

.. code-block:: rst

   .. musicscore:: abc
      :title: Mary Had a Little Lamb
      
      X:1
      T:Mary Had a Little Lamb
      M:4/4
      L:1/4
      K:C
   E D C D | E E E2 | D D D2 | E G G2 |
   E D C D | E E E E | D D E D | C |

Two-part harmony:

.. code-block:: rst

   .. musicscore:: abc
      :title: Simple Duet
      
      X:1
      T:Simple Duet
      M:4/4
      L:1/4
      K:C
   V:1 name="Melody"
   C E G C | G E C2 |
   V:2 name="Bass" clef=bass
   C, G, C, C, | G, C, C,2 |

Related Documentation
--------------------

- :doc:`/features/diagrams/diagrams_design_spec` - Similar architecture reference
- :doc:`/features/codeblocks/codeblock_pipeline` - Asset generation pattern