================
RuSTructuredText
================

.. image:: https://travis-ci.com/flying-sheep/rust-rst.svg?branch=master
   :target: https://travis-ci.com/flying-sheep/rust-rst

Designed around the `Docutils Document Tree`_ and the `reStructuredText specification`_, this is supposed to become a library able to convert reStructuredText and Docutils XML to both each other and HTML5.

This project is dual-licensed under Apache 2.0 and MIT.

.. _Docutils Document Tree: http://docutils.sourceforge.net/docs/ref/doctree.html
.. _reStructuredText specification: http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html

.. note::
   If you are looking for the requirements tracking tool rst (Requirements, Specifications and Tests), have a look at the rst_app package instead.

Inspiration
-----------
The design was inspired by the comrak_ Markdown parser library. The rST grammar was inspired by peg-rst_

.. _comrak: https://github.com/kivikakk/comrak
.. _peg-rst: https://github.com/hhatto/peg-rst
