import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)
os.chdir(dname)

features = []

class Feature:
    def __init__(self, name, info, default, vals):
        self._name = name
        self._info = info
        self._default = default
        self._vals = vals

    def name(self):
        return self._name.replace("_", "-")
    
    def name_env(self):
        return self._name.upper()

    def cargo_info(self, val):
        extra = " (default)" if self._default == val else ""
        if self._info:
            return f"## {self._info}{extra}\n"
        return ""

    def cargo_feature(self, val):
        extra = " # Default" if self._default == val else ""
        return f"{self.cargo_info(val)}{self.name()}-{val} = []{extra}\n"

    def build_default(self):
        return f'    ("{self.name_env()}", {f._default}),\n'


def feature(name, default, min=None, max=None, pow2=None, vals=None, factors=[], info=None):
    if vals is None:
        assert min is not None
        assert max is not None

        vals = set()
        val = min
        while val <= max:
            vals.add(val)
            for f in factors:
                if val*f <= max:
                    vals.add(val*f)
            if (pow2 == True or (isinstance(pow2, int) and val >= pow2)) and val > 0:
                val *= 2
            else:
                val += 1
        vals.add(default)
        vals = sorted(list(vals))

    features.append(Feature(name, info, default, vals))

feature("radio_buffer_size", default=256, min=128, max=256, pow2=True, info="Size of radio buffer in bytes")

# ========= Update Cargo.toml

things = ""
for f in features:
    name = f.name()
    for val in f._vals:
        things += f.cargo_feature(val)
    things += "\n"

SEPARATOR_START = "# BEGIN AUTOGENERATED CONFIG FEATURES\n"
SEPARATOR_END = "# END AUTOGENERATED CONFIG FEATURES\n"
HELP = "# Generated by gen_config.py. DO NOT EDIT.\n"
with open("Cargo.toml", "r") as f:
    data = f.read()
before, data = data.split(SEPARATOR_START, maxsplit=1)
_, after = data.split(SEPARATOR_END, maxsplit=1)
data = before + SEPARATOR_START + HELP + things + SEPARATOR_END + after
with open("Cargo.toml", "w") as f:
    f.write(data)


# ========= Update build.rs

things = ""
for f in features:
    things += f.build_default()

SEPARATOR_START = "// BEGIN AUTOGENERATED CONFIG FEATURES\n"
SEPARATOR_END = "// END AUTOGENERATED CONFIG FEATURES\n"
HELP = "    // Generated by gen_config.py. DO NOT EDIT.\n"
with open("build.rs", "r") as f:
    data = f.read()
before, data = data.split(SEPARATOR_START, maxsplit=1)
_, after = data.split(SEPARATOR_END, maxsplit=1)
data = before + SEPARATOR_START + HELP + \
    things + "    " + SEPARATOR_END + after
with open("build.rs", "w") as f:
    f.write(data)