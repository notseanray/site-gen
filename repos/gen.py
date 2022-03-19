import json

"""
    Really janky script to fetch all repo data from GitHub and format it + put in html/sml
"""

repos = []
# read data from the GitHub curl output
with open("./result", "r") as json_data:
    json_data = json.load(json_data)
    for val in json_data:
        # add the information we want
        repo = [
                val["name"],
                val["language"],
                val["forks_count"],
                val["stargazers_count"],
                val["watchers_count"],
                val["topics"],
                val["created_at"],
                val["updated_at"],
                val["html_url"],
                val["fork"]
                ]
        print(repo)
        repos.append(repo)
# write the output sml/html we want to a file
with open("./projects.sml", "w") as output:
    # basic page header for title
    output.write("[section]\n[header]Projects[/header]\n[n]\n")
    # we can mix my markdown lang and html because of how it's parsed
    # this may be messy, and it is, but it works relatively well
    for val in repos:
        # keep track of info we are adding, if there is none then we shouldn't add anything
        fields = 0
        # make sure to tabulate everything so it doesn't look too ugly
        # when viewed in an IDE
        card = "<card>\n\t<container>\n\t\t<name>\n\t\t\t<a href=\"{}\">".format(val[8]) + val[0] + "</a>\n\t\t</name>\n\t\t[n]\n"
        # if the value is zero/none, we don't want to add it
        if val[1] != None:
            card += "\t\t<topLang> \n\t\t\tTop Language: " + val[1] + "\n\t\t</topLang>"
            fields += 1
        if val[2] != 0:
            card += "\n\t\t[n]Forks: " + str(val[2]) + " "
            fields += 1
        if val[3] != 0:
            card += "\n\t\t[n]Stars: " + str(val[3]) + " "
            fields += 1
        if val[4] != 0:
            card += "\n\t\t[n]Watchers: " + str(val[4]) + " "
            fields += 1
        if len(val[5]) > 0:
            card += "\n\t\t<topics>\n\t\t\t[n]Topic(s): \n\t\t\t\t" + val[5][0]
            fields += 1
            for i, topic in enumerate(val[5]):
                if i == 0:
                    continue
                card += ", " + topic
            card += "\n\t\t</topics>"
        card += "\n\t</container>\n</card>[n]"
        if fields > 0:
            output.write(card + "\n");
    # add back to top button with proper spacing
    output.write("[n]\n[n]\n[btt-button]")
