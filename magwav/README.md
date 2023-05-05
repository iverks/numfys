# Readme

To exam graders: 
- The rust source code is in the src-folder. Business logic happens mostly in system.rs meanwhile system setup happens in main.rs.
- The python source code in the scripts-folder is what was used to make all plots except for the animations of the magnets. The structure is very messy, sorry for that.
- I serialized all my data into the plots-folder. Some of the data is garbage from when i had bugs.

## Notater

- Si is the spin direction
- Hi is the effective field

- Hamil = sum(sum( J times Si dot Sj)) - d_z sum(Si in z direction)^2

- del Si = -gamma / (1 + alpha) Si cross Hi

-  Hi = 1/mu Hamil derived w.respect to Si
-  -2(Si in z direction) in z direction