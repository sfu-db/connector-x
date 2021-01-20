(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl CheckedDiv for BigInt","synthetic":false,"types":[]},{"text":"impl CheckedDiv for BigUint","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T&gt; CheckedDiv for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer + CheckedMul,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_traits"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()