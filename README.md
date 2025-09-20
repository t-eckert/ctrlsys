# `ctrlsys`: Control System Platform

```
         t            ll
          t     r       l      
ccccccc ttttttt rrrrrrr  l  sssss  y     y  sssss                               
 c          t     r    r  l  s      y     y  s                        
  c          t     r       l  sssss  y     y  sssss
   c          t     r       l      s  y     y      s               
	ccccccc    tttt  r       ll sssss  yyyyyyy  sssss
                                              y
                                         yyyyyyy
```


This is my hobby project. It's personal development platform built around Kubernetes -- purposefully overkill. I run it in my homelab. Some of it was developed using Claude. And some of the notes in this readme are still under development. This is just my little chill homeSaaS.

The architecture is based around a control plane. The control plane can be accessed through the API app which interface with the control plane using gRPC. The API is used by both the Console and the CLI.



