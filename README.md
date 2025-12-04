# Media

This is the dx-media source code.

Before doing anything please understand the goal of the prompt and the codebase structure.

And if you can't understand the js pages I have got a html of https://pixabay.com/images/search/nature/ images html at JS.html so please learn from that file and scape images correctly!!!

So, up until now we only did search and download of media files from various providers. But now we are finishing it by listing all the search images in json format froma provier and then for any search we don't just search from only one provier but all providers and scappers use the full power of rust async and concurrency to do the searches to list all responces from all scappers and all providers so that we can use this urls in our cli and our code editor extension and website to show all the images from all providers and all scappers for a given search term. Just focus on the search and listing all url of all media files from all providers and scappers for a given search term. We will do the download part later. SO the main goal is to do search and list all the urls of all media files from all providers and scappers for a given search term!!!

Please learn from CLAUDE.md and Claude_thinkging.md file and add these resounrces in in dx-media!!!

So, from the timing details we can learn from that Unified (--all) and Single Provider are the best options for search. So, for a search make sure when we do unified it will search all providers about images, or audio, or videos like this and single provider search may need an improvement too so please make sure that if can make these faster as we are using rust so use rust programming languages full power to make these searches faster and better!!!

For speed we currently did thi:
```markdown
Key Optimizations Applied:
FuturesUnordered - Results stream as they arrive
5-second timeout (was 8s) - Fast failures
Early Exit - Stop when 3× results gathered
HTTP Connection Pool - 10 keep-alive per host
TCP_NODELAY - Nagle's algorithm disabled
5s Connect Timeout - Quick connection failures
3s Scraper Timeout - Fast web scraping
rayon crate - Ready for future parallel CPU work
The Early Exit is the killer feature - when searching for "monet", we got 16 results from 3 fast providers (LOC, ARTIC, Europeana) and immediately stopped waiting for 5 slower providers!```

So, this is great now please do these things
1. Add two search mode (1. Quantity and other one is Quality) so in quantity mode please put current implementation and it quility mode please wait for all providers to respond and then give the results so that we can have both options for search
2. Add rayon crate in Cargo.toml for future parallel CPU-bound work + Change the scraper timeout from 5 seconds to 3 seconds in src/engine/d for more speed

And try again to make our dx-media to use rust full power to make search faster and better!!!
