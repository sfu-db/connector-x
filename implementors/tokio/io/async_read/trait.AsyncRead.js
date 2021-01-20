(function() {var implementors = {};
implementors["hyper"] = [{"text":"impl AsyncRead for Upgraded","synthetic":false,"types":[]}];
implementors["hyper_tls"] = [{"text":"impl&lt;T:&nbsp;AsyncRead + AsyncWrite + Unpin&gt; AsyncRead for MaybeHttpsStream&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_native_tls"] = [{"text":"impl&lt;S&gt; AsyncRead for TlsStream&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: AsyncRead + AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio_postgres"] = [{"text":"impl AsyncRead for Socket","synthetic":false,"types":[]},{"text":"impl AsyncRead for NoTlsStream","synthetic":false,"types":[]}];
implementors["tokio_util"] = [{"text":"impl&lt;L, R&gt; AsyncRead for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsyncRead,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsyncRead,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()