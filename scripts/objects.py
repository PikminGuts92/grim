#import json
import pikaxe
from typing import Any

tex = pikaxe.RndTex()
#tex.custom_tag = 'whatever'

tex.name = 'whatever.tex'
tex.width = 1024
tex.height = 1024
tex.bpp = 8

#print(json.dumps(tex.__dict__, indent=4))

#def print_obj_properties(obj: pikaxe.MiloObject):
def print_obj_properties(obj: Any):
    print(obj.__class__.__name__)

    for attr_name in dir(obj):
        if attr_name.startswith('_'):
            continue

        attr_value = getattr(obj, attr_name)
        if callable(attr_value):
            continue

        print(f'\t{attr_name}: {attr_value}')

print_obj_properties(tex)

img_size = tex.compute_image_size()
print(f'Image Size: {img_size}')

types = type(tex).__mro__
print(f'Types: {types}')

obj = pikaxe.Object()
obj.name = 'some_name_1'

obj_dir = pikaxe.ObjectDir()
obj_dir.name = 'some_name_2'
obj_dir.type = 'testtest'

print_obj_properties(obj)
print_obj_properties(obj_dir)

#for field, value in vars(tex).items():
#    print(f'{field}: {value}')