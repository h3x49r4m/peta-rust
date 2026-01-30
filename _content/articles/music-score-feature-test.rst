---
title: Music Score Feature Test
date: 2026-01-30T00:00:00
tags: [music, abc-notation, feature-test]
---

Music Score Feature Test
========================

This article demonstrates the music score rendering feature using ABC notation in RST files.

Simple Melody
-------------

Here's a simple melody using ABC notation:

.. musicscore:: abc
   :title: Twinkle Twinkle Little Star
   
   X:1
   T:Twinkle Twinkle Little Star
   M:4/4
   L:1/4
   K:C
   C C G G | A A G2 | F F E E | D D C2 |

Mary Had a Little Lamb
-----------------------

Another classic children's song:

.. musicscore:: abc
   :title: Mary Had a Little Lamb
   
   X:1
   T:Mary Had a Little Lamb
   M:4/4
   L:1/4
   K:C
   E D C D | E E E2 | D D D2 | E G G2 |
   E D C D | E E E E | D D E D | C4 |

Scale Practice
--------------

A simple C major scale:

.. musicscore:: abc
   :title: C Major Scale
   
   X:1
   T:C Major Scale
   M:4/4
   L:1/4
   K:C
   C D E F | G A B c | c B A G | F E D C |

Features
--------

The music score renderer supports:

- **ABC notation**: Text-based music notation format
- **Titles**: Display titles above the music score
- **Staff lines**: Automatic rendering of 5-line staff
- **Clefs**: Treble clef display
- **Time signatures**: Displayed at the beginning of each staff
- **Key signatures**: Automatic key signature display
- **Notes**: Simplified note rendering with stems
- **Bar lines**: Visual separation of measures
- **Download**: Click the download button to save as SVG with copyright

How to Use
----------

To add a music score to your RST content, use the ``musicscore`` directive:

.. code-block:: rst

    .. musicscore:: abc
       :title: Your Song Title
   
        X:1
        T:Song Title
        M:4/4
        L:1/4
        K:C
        C D E F | G A B c |

Supported Options
~~~~~~~~~~~~~~~~~

- ``title``: Display title above the music score (optional)

ABC Notation Basics
-------------------

ABC notation uses simple text characters to represent music:

- ``X``: Index number
- ``T``: Title
- ``M``: Meter (time signature)
- ``L``: Default note length
- ``K``: Key signature
- Notes: ``C``, ``D``, ``E``, ``F``, ``G``, ``A``, ``B`` (lowercase for higher octave)
- Bar lines: ``|``
- Note lengths: ``2`` (half note), ``4`` (quarter note), ``8`` (eighth note)

Future Enhancements
-------------------

Planned features for the music score renderer:

- Full ABC notation support (chords, repeats, multi-measure rests)
- LilyPond notation support
- MusicXML import/export
- Audio playback using Web Audio API
- Transposition capabilities
- Multiple clefs (bass, alto, tenor)
- Multi-staff support for piano and orchestral scores

For more information, see the :doc:`/features/music_scores/music_scores_design_spec`.
