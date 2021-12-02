l,s1,s2,f={"f":lambda u,s:[s[0]+u,s[1]+u*s[2],s[2]],"u":lambda u,s:[s[0],s[1],s[2]-u],"d":lambda u,s:[s[0],s[1],s[2]+u]},{},[0,0,0],open("input.txt","r")
for (c, u) in [(c[0][0],int(c[1])) for c in map(str.split, f.read().splitlines())]:
    (s1[c],s2) = (u + s1.get(c, 0), l[c](u, s2))
f.close() or print(s1["f"]*(s1["d"]-s1["u"]),s2[0]*s2[1])
