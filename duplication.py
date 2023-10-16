import os

# Store data folder
data_folder = "./sysprog_p1_stevenkight/src/data"

# Store all files in data folder
files = os.listdir(data_folder + "/ALBNM")

# Fake branch names
branches = ["ATLAGA", "SAVAGA", "MACOGA", "AGUSGA", "COLUGA", 
            "BIRBGA", "BIR1CGA", "BIR2CGA", "BIR3CGA", "BIR4CGA",
            "RALEGA", "DURHGA", "CHA1GA", "CHA2GA", "CHA3GA",
            "CHA4GA", "CHA5GA", "CHA6GA", "CHA7GA", "CHA8GA",
            "CHA9GA", "CHA0GA", "CHAAGA", "CHABGA", "CHACGA",
            "CHADGA", "CHAEGA", "CHA1EGA", "CHAFGA", "CHAGGA",
            "CHAHGA", "CHA1HGA", "CHA2HGA", "CHA3HGA", "CHA4HGA",
            "CHA5HGA", "CHA6HGA", "CHA7HGA", "CHA8HGA", "CHA9HGA"]

for branch in branches:
    if (os.path.exists(data_folder + "/" + branch)):
        continue

    # Create new folder for branch
    os.mkdir(data_folder + "/" + branch)
    for file in files:
        os.system("cp " + data_folder + "/ALBNM/" + file + " " + data_folder + "/" + branch + "/" + file)

    # Replace all instances of ALBNM with branch name
    for file in files:
        with open(data_folder + "/" + branch + "/" + file, "r") as f:
            contents = f.read()
        contents = contents.replace("ALBNM", branch)
        with open(data_folder + "/" + branch + "/" + file, "w") as f:
            f.write(contents)
