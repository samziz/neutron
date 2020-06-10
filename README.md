# Neutron: An alternative Web (with browser included)


*This is veeery much a work in progress!*

# Installing

I don't have any pre-built binaries at the moment. You'll need to install Cargo and run `cargo run`.

# Rationale

This is my experimental web browser, aimed at fixing all the flaws (I perceived) in the existing model of the Web. It uses neither HTML, CSS, nor JavaScript. It aims to provide a few advantages over the current crop of browser vendors:

- Much improved page load and interaction speed.
- Richer online experiences 'for less'.
- Easy peer to peer communication, where this is easier (or indeed more secure) than communicating via a central web server.

## Faults with the modern web

### Browser heterogeneity wastes developer time and causes bugs

- With many users using antiquated or unusual/annoying browsers (cf. Internet Explorer), developers have to spend a great deal of time polyfilling their code and [thinking about compatibility](https://caniuse.com/).

### CSS 

The CSS box model is [confused](https://pressupinc.com/blog/2014/01/whats-wrong-css-box-model-fix/), mixing absolute dimensions with flexbox-style responsive design.

In addition to this, the cascading logic makes it impossible to style element B based on some property of element A unless element A happens to be a sibling or child of element B. This leads developers to write hacky solutions using JavaScript - which, as mentioned above, plenty of users now disable, and which in either case are prone to breaking.

### HTML

#### Confusing layout

It's not immediately apparently how various element types will lay out their children, hence the [table era](http://www.barrypearson.co.uk/articles/layout_tables/history.htm) of HTML. 

#### Very limited without JavaScript

HTML is hamstrung without JavaScript. However, an increasing number of users don't want to support JavaScript because of privacy concerns. These are some limitations of HTML when JS is disabled.

- HTML requires JS in order to support lots of UI functionality. 

- HTML requires JS in order to process and send complicated forms: e.g. fields in a pure HTML form are sent form-encoded, which makes it hard to represent e.g. array or map data structures in forms.

- HTML cannot send PUT or DELETE requests without resorting to JavaScript, which undercuts the purpose of REST and forces developers to use awkward hacks that mean the browser can't correctly judge e.g. the idempotency of a request.

- Errors returned from HTML POST forms have no standard way of being rendered. You can render the `Referrer` with an error parameter in the query string, but what about all the other parameters required to load the page.

- GETs, in spite of the HTML RFC, do often have side effects (and this is sometimes desirable, such as when you want to count the number of loads on a page). This means browser vendors are [often hesitant to pre-fetch](https://en.wikipedia.org/wiki/Link_prefetching#Browser_support) and will only do so under certain circumstances, leading to a slower experience for the user.

### JavaScript

- JavaScript, although the established language of web (all others, save WebAssembly, compile to it), is [widely hated](https://news.ycombinator.com/item?id=20807953) for its inconsistent and unintuitive semantics.

- The wide use of **third-party** JavaScript (i.e. scripts requested by a site from another domain) opens the door for vulnerabilities such as XSS. JavaScript has no fine-grained control system preventing certain dependencies from doing dangerous actions like reading sensitive data from DOM elements and making network calls.

In short, the current situation with JavaScript is a bit like if you had to choose between leaving your door open all the time or closed all the time. Leaving it open means allowing a wide range of shocking and [shockingly easy vulnerabilities](https://medium.com/hackernoon/im-harvesting-credit-card-numbers-and-passwords-from-your-site-here-s-how-9a8cb347c5b5), many of them supply chain vulnerabilities, and which can be achieved in spite of a developer's best efforts in setting a sensible CSP etc.

- The script loading process is badly implemented. It may be that script A and B can load right away, but script C depends on script A, and then script D depends on script A. The developer will want to minimise page load time, especially if the scripts are large and/or critical, but HTML offers no way of expressing this kind of dependency graph.

## The happy future*

Neutron is an experimental browser for the new web which does the following:

### Near-instant speed with intelligent pre-fetching

A link element will have an `is_idempotent` attribute so the browser knows in all cases when it can safely pre-load a site and pre-render its images in the background (saving them to disk, which is cheap nowadays). Most of this work will happen in a non-blocking fashion, and so it won't affect browsing experience on the current page, but it means new page loads will be near-instant.

### Sandboxed dependencies

It should be possible to do the equivalent of the following:

```
<script src="https://google.com/track-all-the-things.js" allowNetwork=false allowDomAccess=false />
```

to restrict the dangers of importing untrusted third-party scripts.

### More intelligent forms

Forms should be able to be structured in such a way that they don't need to be cleverly processed using JavaScript in order to send a complex data structure to the server. At a stretch, such a browser could support state management natively within HTML (or whatever its equivalent is), avoiding the JavaScript conundrum.

### Better error messages

The browser should (without recourse to JavaScript) automatically render errors triggered by HTTP requests in a clear way (with the exact formatting specified in its style configuration: the bit that's responsible for what CSS currently does).

### Streaming site content

The language used should make it easy for browsers to receive the most essential (i.e. 'above-the-fold' or contextually relevant content) first. The browser will then process the response as it's streamed.

### Straightforward inbuilt APIs for interactions such as video streaming

HTML does not have a rich range of elements for interactions like two-way video streaming using cameras (in other words, video calling).

### Sophisticated, intelligent plugins

It should be possible to install a plugin to your browser, written by e.g. [Stripe](https://stripe.com/gb?utm_campaign=paid_brand-UK_en_Search_Brand_Stripe-2032860449&utm_medium=cpc&utm_source=google&ad_content=355351450259&utm_term=stripe&utm_matchtype=e&utm_adposition=&utm_device=c&gclid=EAIaIQobChMIg4Tb16v36QIV2O3tCh3u6wouEAAYASAAEgLb6vD_BwE). Such plugins would allow persistent (or configurable-duration) authentication and mean users can pay and perform other actions across the web without needing to be ping-ponged between different sites.


\* OK, I stole this phrase from my days at [Monzo](https://monzo.com/).
