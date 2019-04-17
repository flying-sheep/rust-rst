Development
===========

To format the HTML, you can use dindent_.
I didn’t find anything else that *just* reindents HTML.

.. code:: bash

   cargo run -- README.rst | dindent --input='php://stdin'

.. _dindent: https://github.com/gajus/dindent
