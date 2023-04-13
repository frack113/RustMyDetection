find.exe -f (objectcategory=person) >  ad_users.txt
find.exe -f objectcategory=computer > ad_computers.txt
find.exe -f (objectcategory=organizationalUnit) > ad_ous.txt
find.exe -subnets -f (objectCategory=subnet) > ad_subnets.txt
find.exe -f "(objectcategory=group)" > ad_group.txt
find.exe -gcb -sc trustdmp >  ad_trustdmp.txt
7za.exe a -mx3 ad.7z ad_*
del *.txt