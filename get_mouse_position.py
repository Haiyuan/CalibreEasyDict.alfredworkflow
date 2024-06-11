import Quartz.CoreGraphics as CG

loc = CG.CGEventGetLocation(CG.CGEventCreate(None))
print(int(loc.x), int(loc.y))