# inspire_hep_rss feed

This simple rust program queries the inspireHEP REST API and provides a RSS feed under e.g. http://127.0.0.1:3000/?sort=mostrecent&size=10&q=a%20Alexander.Neuwirth.1 following the options of the API https://github.com/inspirehep/rest-api-doc.

When adding the RSS feed to e.g. Thunderbird, please keep the update frequency low, like 1 update per day.

Below image shows the feed in Thunderbird.

![Thunderbird](./img/view.png)

Thus, it serves as a simple way to keep track of new publications of a given author.
