from py_codeowners import PyCodeowners

owners = PyCodeowners("""
* @platform
team-a/** @team-a
"""
)
print(owners.of("team-a/test.js"))

owners = PyCodeowners.from_path("./testdata/CODEOWNERS")
print(owners.of("team-a/test.js"))