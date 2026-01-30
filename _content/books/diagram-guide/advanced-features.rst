Advanced Features
=================

This chapter covers advanced features and techniques for creating more sophisticated diagrams in Peta.

Title Styling
-------------

Custom Titles
~~~~~~~~~~~~~

All diagram types support custom titles:

.. code-block:: rst

   .. diagram:: flowchart
      :title: My Custom Title

      Start -> End

The title appears at the top of the diagram and helps identify its purpose.

Title Best Practices
~~~~~~~~~~~~~~~~~~~~~

- Keep titles concise (under 10 words)
- Make titles descriptive
- Use consistent capitalization
- Avoid special characters
- Align titles with content

Diagram Composition
-------------------

Combining Diagrams
~~~~~~~~~~~~~~~~~~

You can use multiple diagrams in a single document to show different aspects:

Example 1: High-Level Flow
---------------------------

.. diagram:: flowchart
   :title: System Overview
   
   User -> System -> Database

Example 2: Detailed Process
----------------------------

.. diagram:: flowchart
   :title: Detailed Process
   
   User -> Login -> Validate -> Database
   Validate -> Show Error -> Login

This approach helps organize complex information.

Integration with RST Features
------------------------------

Code Blocks
~~~~~~~~~~~

Combine diagrams with code examples:

.. diagram:: flowchart
   :title: Algorithm Flow
   
   Input -> Process -> Output

.. code-block:: python

   def process(input):
       result = transform(input)
       return result

Math Formulas
~~~~~~~~~~~~~

Use diagrams alongside mathematical expressions:

.. diagram:: flowchart
   :title: Optimization Process
   
   Initial Point -> Calculate Gradient -> Update Point -> Check Convergence

The gradient descent formula: $\theta_{new} = \theta_{old} - \alpha \nabla J(\theta)$

Article References
~~~~~~~~~~~~~~~~~~

Link to related content:

.. diagram:: sequence
   :title: Authentication Flow
   
   User -> System: Login
   System -> Auth: Validate

.. article-card:: quantum-mechanics

Authentication principles are similar to quantum state verification in certain aspects.

Snippets
~~~~~~~~

Reference code snippets:

.. diagram:: class-diagram
   :title: System Architecture
   
   Controller |+| Service

.. snippet-card:: python-data-processing

The service layer processes data as shown in the example above.

Styling Considerations
----------------------

Visual Hierarchy
~~~~~~~~~~~~~~~~

Create visual hierarchy using:
- Diagram titles
- Section headings
- Different diagram types
- Spacing and layout

Example:

High-Level Architecture
-----------------------

.. diagram:: class-diagram
   :title: System Components
   
   App |+| Service

Implementation Details
----------------------

.. diagram:: flowchart
   :title: Processing Flow
   
   Start -> Process -> End

Consistency
~~~~~~~~~~~

Maintain consistency across diagrams:
- Use similar naming conventions
- Apply consistent styling
- Follow logical organization
- Keep diagrams balanced

Performance Considerations
---------------------------

Optimization Tips
~~~~~~~~~~~~~~~~~

1. **Limit complexity**: Avoid overly complex diagrams
2. **Use appropriate detail**: Match detail level to purpose
3. **Optimize file size**: Keep diagrams concise
4. **Test frequently**: Verify diagrams render correctly

Build Performance
~~~~~~~~~~~~~~~~~

Diagram rendering is optimized by:
- Server-side processing
- Static SVG generation
- No client-side rendering
- Efficient caching

Accessibility
-------------

Semantic Structure
~~~~~~~~~~~~~~~~~~

Use proper RST structure:

.. code-block:: rst

   .. diagram:: flowchart
      :title: Accessible Flowchart

      Start -> End

The title provides context for screen readers.

Descriptive Content
~~~~~~~~~~~~~~~~~~~

Include descriptive text around diagrams:

This flowchart shows the user registration process. The flow moves from the initial form submission through validation to account creation.

Alternative Text
~~~~~~~~~~~~~~~~~

The diagram title serves as alternative text for accessibility purposes.

Common Use Cases
----------------

Documentation
~~~~~~~~~~~~~

Technical documentation often benefits from diagrams:

.. diagram:: flowchart
   :title: API Request Flow
   
   Client -> Gateway -> Service -> Database

Presentations
~~~~~~~~~~~~~

Diagrams enhance presentations:

.. diagram:: gantt
   :title: Project Timeline
   
   Phase 1 [2024-01-01] : 10d
   Phase 2 [2024-01-11] : 15d

Education
~~~~~~~~~

Educational content uses diagrams for clarity:

.. diagram:: sequence
   :title: Learning Process
   
   Student -> Teacher: Question
   Teacher -> Student: Explanation

Troubleshooting
===============

Common Issues
-------------

Build Failures
~~~~~~~~~~~~~~

If the build fails:
1. Check diagram syntax
2. Verify diagram type is correct
3. Ensure proper indentation
4. Review error messages

Rendering Issues
~~~~~~~~~~~~~~~~

If diagrams don't render correctly:
1. Validate syntax against examples
2. Check for special characters
3. Verify title formatting
4. Test with simpler diagrams first

Getting Help
------------

Resources
~~~~~~~~~

- Check syntax examples in this book
- Review the diagram test article
- Examine existing diagrams in the codebase
- Consult the feature documentation

Debugging Tips
~~~~~~~~~~~~~

1. Start with minimal diagrams
2. Add complexity gradually
3. Test frequently during development
4. Use the build output for diagnostics
5. Compare with working examples

Next Steps
----------

The final chapter covers best practices and tips for creating effective diagrams.