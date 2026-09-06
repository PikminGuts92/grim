import pikaxe as px

mf = px.MiloFile.load_from_file('./test.milo')

trans = px.RndTransformable()
trans.local_xfm.translation.x = 10.0
trans.local_xfm.rotation.pitch = 90.0

mf.object_dir.entries.append(trans)