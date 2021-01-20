(function() {var implementors = {};
implementors["num_bigint"] = [{"text":"impl Num for BigInt","synthetic":false,"types":[]},{"text":"impl Num for BigUint","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Num + Clone&gt; Num for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; Num for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_traits"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()