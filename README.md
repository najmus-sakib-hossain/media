# Media

This is the dx-media source code.

Before doing anything please understand the goal of the prompt and the codebase structure.

So, up until now we only did search and download of media files from various providers. But now we are finishing it by listing all the search images in json format froma provier and then for any search we don't just search from only one provier but all providers and scappers use the full power of rust async and concurrency to do the searches to list all responces from all scappers and all providers so that we can use this urls in our cli and our code editor extension and website to show all the images from all providers and all scappers for a given search term. Just focus on the search and listing all url of all media files from all providers and scappers for a given search term. We will do the download part later. SO the main goal is to do search and list all the urls of all media files from all providers and scappers for a given search term!!!
