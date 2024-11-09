# inspire_hep_rss feed

This simple rust program fetches the inspire hep rest and provides a rss feed under e.g. http://127.0.0.1:3000/?sort=mostrecent&size=10&q=a%20Alexander.Neuwirth.1 following the options of the API https://github.com/inspirehep/rest-api-doc.

When adding the RSS feed to e.g. Thunderbird, please keep the update frequency low, like 1 update per day.

Below image shows the feed in Thunderbird.

![Thunderbird](./src/view.png)