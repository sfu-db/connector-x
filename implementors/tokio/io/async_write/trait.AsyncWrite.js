(function() {var implementors = {};
implementors["hyper"] = [{"text":"impl AsyncWrite for Upgraded","synthetic":false,"types":[]}];
implementors["hyper_tls"] = [{"text":"impl&lt;T:&nbsp;AsyncWrite + AsyncRead + Unpin&gt; AsyncWrite for MaybeHttpsStream&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_native_tls"] = [{"text":"impl&lt;S&gt; AsyncWrite for TlsStream&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: AsyncRead + AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio_postgres"] = [{"text":"impl AsyncWrite for Socket","synthetic":false,"types":[]},{"text":"impl AsyncWrite for NoTlsStream","synthetic":false,"types":[]}];
implementors["tokio_util"] = [{"text":"impl&lt;L, R&gt; AsyncWrite for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: AsyncWrite,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AsyncWrite,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()