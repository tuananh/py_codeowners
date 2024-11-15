from py_codeowners import PyCodeowners

owners = PyCodeowners("./testdata/CODEOWNERS")
print(owners.of("team-a/test.js"))
