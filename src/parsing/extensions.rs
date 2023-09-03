use std::path::Path;

use regex::Regex;

fn is_valid_extension(tested_extension: &str) -> bool {
    let valid_extension = vec![
        ".php".to_string(),
        ".html".to_string(),
        ".txt".to_string(),
        ".htm".to_string(),
        ".aspx".to_string(),
        ".asp".to_string(),
        ".js".to_string(),
        ".css".to_string(),
        ".pgsql.txt".to_string(),
        ".mysql.txt".to_string(),
        ".pdf".to_string(),
        ".cgi".to_string(),
        ".inc".to_string(),
        ".gif".to_string(),
        ".jpg".to_string(),
        ".swf".to_string(),
        ".xml".to_string(),
        ".cfm".to_string(),
        ".xhtml".to_string(),
        ".wmv".to_string(),
        ".zip".to_string(),
        ".axd".to_string(),
        ".gz".to_string(),
        ".png".to_string(),
        ".doc".to_string(),
        ".shtml".to_string(),
        ".jsp".to_string(),
        ".ico".to_string(),
        ".exe".to_string(),
        ".csi".to_string(),
        ".inc.php".to_string(),
        ".config".to_string(),
        ".jpeg".to_string(),
        ".ashx".to_string(),
        ".log".to_string(),
        ".xls".to_string(),
        ".old".to_string(),
        ".mp3".to_string(),
        ".com".to_string(),
        ".tar".to_string(),
        ".ini".to_string(),
        ".asa".to_string(),
        ".tgz".to_string(),
        ".PDF".to_string(),
        ".flv".to_string(),
        ".php3".to_string(),
        ".bak".to_string(),
        ".rar".to_string(),
        ".asmx".to_string(),
        ".xlsx".to_string(),
        ".page".to_string(),
        ".phtml".to_string(),
        ".dll".to_string(),
        ".JPG".to_string(),
        ".asax".to_string(),
        ".msg".to_string(),
        ".pl".to_string(),
        ".GIF".to_string(),
        ".ZIP".to_string(),
        ".csv".to_string(),
        ".css.aspx".to_string(),
        ".JPEG".to_string(),
        ".ppt".to_string(),
        ".nsf".to_string(),
        ".Pdf".to_string(),
        ".Gif".to_string(),
        ".bmp".to_string(),
        ".sql".to_string(),
        ".Jpeg".to_string(),
        ".Jpg".to_string(),
        ".xml.gz".to_string(),
        ".Zip".to_string(),
        ".new".to_string(),
        ".avi".to_string(),
        ".psd".to_string(),
        ".rss".to_string(),
        ".wav".to_string(),
        ".action".to_string(),
        ".db".to_string(),
        ".dat".to_string(),
        ".do".to_string(),
        ".xsl".to_string(),
        ".class".to_string(),
        ".mdb".to_string(),
        ".include".to_string(),
        ".cs".to_string(),
        ".class.php".to_string(),
        ".htc".to_string(),
        ".mov".to_string(),
        ".tpl".to_string(),
        ".js.php".to_string(),
        ".mysql-connect".to_string(),
        ".mpg".to_string(),
        ".rdf".to_string(),
        ".rtf".to_string(),
        ".ascx".to_string(),
        ".mvc".to_string(),
        ".1.0".to_string(),
        ".files".to_string(),
        ".master".to_string(),
        ".jar".to_string(),
        ".vb".to_string(),
        ".mp4".to_string(),
        ".local.php".to_string(),
        ".fla".to_string(),
        ".require".to_string(),
        ".de".to_string(),
        ".docx".to_string(),
        ".php5".to_string(),
        ".wci".to_string(),
        ".readme".to_string(),
        ".cfg".to_string(),
        ".aspx.cs".to_string(),
        ".cfc".to_string(),
        ".dwt".to_string(),
        ".ru".to_string(),
        ".LCK".to_string(),
        ".Config".to_string(),
        ".gif_var_DE".to_string(),
        ".html_var_DE".to_string(),
        ".net".to_string(),
        ".ttf".to_string(),
        ".HTM".to_string(),
        ".X-AOM".to_string(),
        ".jhtml".to_string(),
        ".mpeg".to_string(),
        ".ASP".to_string(),
        ".LOG".to_string(),
        ".X-FANCYCAT".to_string(),
        ".php4".to_string(),
        ".readme_var_DE".to_string(),
        ".vcf".to_string(),
        ".X-RMA".to_string(),
        ".X-AFFILIATE".to_string(),
        ".X-OFFERS".to_string(),
        ".X-AFFILIATE_var_DE".to_string(),
        ".X-AOM_var_DE".to_string(),
        ".X-FANCYCAT_var_DE".to_string(),
        ".X-FCOMP".to_string(),
        ".X-FCOMP_var_DE".to_string(),
        ".X-GIFTREG".to_string(),
        ".X-GIFTREG_var_DE".to_string(),
        ".X-MAGNIFIER".to_string(),
        ".X-MAGNIFIER_var_DE".to_string(),
        ".X-OFFERS_var_DE".to_string(),
        ".X-PCONF".to_string(),
        ".X-PCONF_var_DE".to_string(),
        ".X-RMA_var_DE".to_string(),
        ".X-SURVEY".to_string(),
        ".tif".to_string(),
        ".dir".to_string(),
        ".json".to_string(),
        ".6.9".to_string(),
        ".Zif".to_string(),
        ".wma".to_string(),
        ".mid".to_string(),
        ".rm".to_string(),
        ".aspx.vb".to_string(),
        ".tar.gz".to_string(),
        ".woa".to_string(),
        ".main".to_string(),
        ".ram".to_string(),
        ".opml".to_string(),
        ".0.html".to_string(),
        ".css.php".to_string(),
        ".feed".to_string(),
        ".lasso".to_string(),
        ".6.3".to_string(),
        ".shtm".to_string(),
        ".sitemap".to_string(),
        ".scc".to_string(),
        ".tmp".to_string(),
        ".backup".to_string(),
        ".sln".to_string(),
        ".org".to_string(),
        ".conf".to_string(),
        ".mysql-query".to_string(),
        ".session-start".to_string(),
        ".uk".to_string(),
        ".TXT".to_string(),
        ".orig".to_string(),
        ".settings.php".to_string(),
        ".cab".to_string(),
        ".kml".to_string(),
        ".lck".to_string(),
        ".pps".to_string(),
        ".require-once".to_string(),
        ".asx".to_string(),
        ".bok".to_string(),
        ".msi".to_string(),
        ".c".to_string(),
        ".fcgi".to_string(),
        ".fopen".to_string(),
        ".html.".to_string(),
        ".phpmailer.php".to_string(),
        ".bin".to_string(),
        ".htaccess".to_string(),
        ".info".to_string(),
        ".java".to_string(),
        ".jsf".to_string(),
        ".tmpl".to_string(),
        ".DOC".to_string(),
        ".bat".to_string(),
        ".com.html".to_string(),
        ".print".to_string(),
        ".resx".to_string(),
        ".ics".to_string(),
        ".php.php".to_string(),
        ".x".to_string(),
        ".PNG".to_string(),
        ".data".to_string(),
        ".dcr".to_string(),
        ".enfinity".to_string(),
        ".html.html".to_string(),
        ".licx".to_string(),
        ".mno".to_string(),
        ".plx".to_string(),
        ".vm".to_string(),
        ".5.php".to_string(),
        ".HTML".to_string(),
        ".MP3".to_string(),
        ".config.php".to_string(),
        ".dwg".to_string(),
        ".edu".to_string(),
        ".search".to_string(),
        ".static".to_string(),
        ".wws".to_string(),
        ".6.edu".to_string(),
        ".OLD".to_string(),
        ".bz2".to_string(),
        ".co.uk".to_string(),
        ".ece".to_string(),
        ".epc".to_string(),
        ".getimagesize".to_string(),
        ".ice".to_string(),
        ".it_Backup_Giornaliero".to_string(),
        ".it_Backup_Settimanale".to_string(),
        ".jspa".to_string(),
        ".lst".to_string(),
        ".php-dist".to_string(),
        ".svc".to_string(),
        ".vbs".to_string(),
        ".1.html".to_string(),
        ".30-i486".to_string(),
        ".ai".to_string(),
        ".cur".to_string(),
        ".dmg".to_string(),
        ".img".to_string(),
        ".inf".to_string(),
        ".seam".to_string(),
        ".smtp.php".to_string(),
        ".1-bin-Linux-2.0.30-i486".to_string(),
        ".7z".to_string(),
        ".ajax".to_string(),
        ".cfm.cfm".to_string(),
        ".chm".to_string(),
        ".csp".to_string(),
        ".edit".to_string(),
        ".file".to_string(),
        ".gif.php".to_string(),
        ".m3u".to_string(),
        ".psp".to_string(),
        ".py".to_string(),
        ".sh".to_string(),
        ".test".to_string(),
        ".zdat".to_string(),
        ".admin".to_string(),
        ".captcha.aspx".to_string(),
        ".dev".to_string(),
        ".eps".to_string(),
        ".file-get-contents".to_string(),
        ".fr".to_string(),
        ".fsockopen".to_string(),
        ".list".to_string(),
        ".m4v".to_string(),
        ".min.js".to_string(),
        ".new.html".to_string(),
        ".p".to_string(),
        ".store".to_string(),
        ".webinfo".to_string(),
        ".xml.php".to_string(),
        ".BAK".to_string(),
        ".htm.".to_string(),
        ".php.bak".to_string(),
        ".bk".to_string(),
        ".bsp".to_string(),
        ".cms".to_string(),
        ".csshandler.ashx".to_string(),
        ".d".to_string(),
        ".html,".to_string(),
        ".htmll".to_string(),
        ".idx".to_string(),
        ".images".to_string(),
        ".jad".to_string(),
        ".master.cs".to_string(),
        ".prev_next".to_string(),
        ".ssf".to_string(),
        ".stm".to_string(),
        ".txt.gz".to_string(),
        ".Web.UI.WebResource.axd".to_string(),
        ".as".to_string(),
        ".asp.asp".to_string(),
        ".au".to_string(),
        ".cnf".to_string(),
        ".dhtml".to_string(),
        ".enu".to_string(),
        ".html.old".to_string(),
        ".include-once".to_string(),
        ".lock".to_string(),
        ".m".to_string(),
        ".mysql-select-db".to_string(),
        ".phps".to_string(),
        ".pm".to_string(),
        ".pptx".to_string(),
        ".sav".to_string(),
        ".sendtoafriendform".to_string(),
        ".ssi".to_string(),
        ".suo".to_string(),
        ".vbproj".to_string(),
        ".wml".to_string(),
        ".xsd".to_string(),
        ".26.13.391N35.50.38.816".to_string(),
        ".26.24.165N35.50.24.134".to_string(),
        ".26.56.247N35.52.03.605".to_string(),
        ".27.02.940N35.49.56.075".to_string(),
        ".27.15.919N35.52.04.300".to_string(),
        ".27.29.262N35.47.15.083".to_string(),
        ".3gp".to_string(),
        ".40.00.573N35.42.57.445".to_string(),
        ".43.58.040N35.38.35.826".to_string(),
        ".44.04.344N35.38.35.077".to_string(),
        ".44.08.714N35.39.08.499".to_string(),
        ".44.10.892N35.38.49.246".to_string(),
        ".44.27.243N35.41.29.367".to_string(),
        ".44.29.976N35.37.51.790".to_string(),
        ".44.32.445N35.36.10.206".to_string(),
        ".44.34.800N35.38.08.156".to_string(),
        ".44.37.128N35.40.54.403".to_string(),
        ".44.40.556N35.40.53.025".to_string(),
        ".44.45.013N35.38.36.211".to_string(),
        ".44.46.104N35.38.22.970".to_string(),
        ".44.48.130N35.38.25.969".to_string(),
        ".44.52.162N35.38.50.456".to_string(),
        ".44.58.315N35.38.53.455".to_string(),
        ".45.01.562N35.38.38.778".to_string(),
        ".45.04.359N35.38.39.112".to_string(),
        ".45.06.789N35.38.22.556".to_string(),
        ".45.10.717N35.38.41.989".to_string(),
        ".ASPX".to_string(),
        ".JS".to_string(),
        ".PHP".to_string(),
        ".array-keys".to_string(),
        ".atom".to_string(),
        ".award".to_string(),
        ".bkp".to_string(),
        ".crt".to_string(),
        ".default".to_string(),
        ".eml".to_string(),
        ".epl".to_string(),
        ".fancybox".to_string(),
        ".fil".to_string(),
        ".geo".to_string(),
        ".h".to_string(),
        ".hmtl".to_string(),
        ".html.bak".to_string(),
        ".ida".to_string(),
        ".implode".to_string(),
        ".index.php".to_string(),
        ".iso".to_string(),
        ".kmz".to_string(),
        ".mysql-pconnect".to_string(),
        ".php.old".to_string(),
        ".php.txt".to_string(),
        ".rec".to_string(),
        ".storefront".to_string(),
        ".taf".to_string(),
        ".war".to_string(),
        ".xslt".to_string(),
        ".1.6".to_string(),
        ".2a".to_string(),
        ".8.1".to_string(),
        ".CSS".to_string(),
        ".NSF".to_string(),
        ".Sponsors".to_string(),
        ".a".to_string(),
        ".aquery".to_string(),
        ".ascx.cs".to_string(),
        ".cat".to_string(),
        ".contrib".to_string(),
        ".ds".to_string(),
        ".dwf".to_string(),
        ".film".to_string(),
        ".g".to_string(),
        ".go".to_string(),
        ".googlebook".to_string(),
        ".gpx".to_string(),
        ".hotelName".to_string(),
        ".htm.htm".to_string(),
        ".ihtml".to_string(),
        ".in-array".to_string(),
        ".index".to_string(),
        ".ini.php".to_string(),
        ".layer".to_string(),
        ".maninfo".to_string(),
        ".odt".to_string(),
        ".price".to_string(),
        ".randomhouse".to_string(),
        ".read".to_string(),
        ".ru-tov.html".to_string(),
        ".s7".to_string(),
        ".sample".to_string(),
        ".sit".to_string(),
        ".src".to_string(),
        ".tpl.php".to_string(),
        ".trck".to_string(),
        ".uguide".to_string(),
        ".vorteil".to_string(),
        ".wbp".to_string(),
        ".2.html".to_string(),
        ".AVI".to_string(),
        ".Asp".to_string(),
        ".EXE".to_string(),
        ".WMV".to_string(),
        ".asax.vb".to_string(),
        ".aspx.aspx".to_string(),
        ".btr".to_string(),
        ".cer".to_string(),
        ".common.php".to_string(),
        ".de.html".to_string(),
        ".html‎".to_string(),
        ".jbf".to_string(),
        ".lbi".to_string(),
        ".lib.php".to_string(),
        ".lnk".to_string(),
        ".login".to_string(),
        ".login.php".to_string(),
        ".mhtml".to_string(),
        ".mpl".to_string(),
        ".mso".to_string(),
        ".mysql-result".to_string(),
        ".original".to_string(),
        ".pgp".to_string(),
        ".ph".to_string(),
        ".php.".to_string(),
        ".preview".to_string(),
        ".preview-content.php".to_string(),
        ".search.htm".to_string(),
        ".site".to_string(),
        ".text".to_string(),
        ".view".to_string(),
        ".3.html".to_string(),
        ".4.html".to_string(),
        ".5.html".to_string(),
        ".ICO".to_string(),
        ".Web".to_string(),
        ".XLS".to_string(),
        ".action2".to_string(),
        ".asc".to_string(),
        ".asp.bak".to_string(),
        ".aspx.resx".to_string(),
        ".browse".to_string(),
        ".code".to_string(),
        ".com_Backup_Giornaliero".to_string(),
        ".com_Backup_Settimanale".to_string(),
        ".csproj".to_string(),
        ".dtd".to_string(),
        ".en.html".to_string(),
        ".ep".to_string(),
        ".eu".to_string(),
        ".form".to_string(),
        ".html1".to_string(),
        ".inc.asp".to_string(),
        ".index.html".to_string(),
        ".it".to_string(),
        ".nl".to_string(),
        ".ogg".to_string(),
        ".old.php".to_string(),
        ".old2".to_string(),
        ".opendir".to_string(),
        ".out".to_string(),
        ".pgt".to_string(),
        ".php,".to_string(),
        ".php‎".to_string(),
        ".po".to_string(),
        ".prt".to_string(),
        ".query".to_string(),
        ".rb".to_string(),
        ".rhtml".to_string(),
        ".ru.html".to_string(),
        ".save".to_string(),
        ".search.php".to_string(),
        ".t".to_string(),
        ".wsdl".to_string(),
        ".0-to1.2.php".to_string(),
        ".CFM".to_string(),
        ".MOV".to_string(),
        ".MPEG".to_string(),
        ".Master".to_string(),
        ".PPT".to_string(),
        ".TTF".to_string(),
        ".Templates".to_string(),
        ".XML".to_string(),
        ".adp".to_string(),
        ".ajax.php".to_string(),
        ".apsx".to_string(),
        ".asf".to_string(),
        ".bck".to_string(),
        ".bu".to_string(),
        ".calendar".to_string(),
        ".captcha".to_string(),
        ".cart".to_string(),
        ".com.crt".to_string(),
        ".core".to_string(),
        ".dict.php".to_string(),
        ".dot".to_string(),
        ".egov".to_string(),
        ".en.php".to_string(),
        ".eot".to_string(),
        ".errors".to_string(),
        ".f4v".to_string(),
        ".fr.html".to_string(),
        ".git".to_string(),
        ".ht".to_string(),
        ".hta".to_string(),
        ".html.LCK".to_string(),
        ".html.printable".to_string(),
        ".ini.sample".to_string(),
        ".lib".to_string(),
        ".lic".to_string(),
        ".map".to_string(),
        ".master.vb".to_string(),
        ".mi".to_string(),
        ".mkdir".to_string(),
        ".o".to_string(),
        ".p7b".to_string(),
        ".pac".to_string(),
        ".parse.errors".to_string(),
        ".pd".to_string(),
        ".pfx".to_string(),
        ".php2".to_string(),
        ".php_files".to_string(),
        ".phtm".to_string(),
        ".png.php".to_string(),
        ".portal".to_string(),
        ".printable".to_string(),
        ".psql".to_string(),
        ".pub".to_string(),
        ".q".to_string(),
        ".ra".to_string(),
        ".reg".to_string(),
        ".restrictor.php".to_string(),
        ".rpm".to_string(),
        ".strpos".to_string(),
        ".tcl".to_string(),
        ".template".to_string(),
        ".tiff".to_string(),
        ".tv".to_string(),
        ".us".to_string(),
        ".user".to_string(),
        ".Controls".to_string(),
        ".WAV".to_string(),
        ".acgi".to_string(),
        ".alt".to_string(),
        ".array-merge".to_string(),
        ".back".to_string(),
        ".call-user-func-array".to_string(),
        ".cfml".to_string(),
        ".cmd".to_string(),
        ".cocomore.txt".to_string(),
        ".detail".to_string(),
        ".disabled".to_string(),
        ".dist.php".to_string(),
        ".djvu".to_string(),
        ".dta".to_string(),
        ".e".to_string(),
        ".extract".to_string(),
        ".file-put-contents".to_string(),
        ".fpl".to_string(),
        ".framework".to_string(),
        ".fread".to_string(),
        ".htm.LCK".to_string(),
        ".inc.js".to_string(),
        ".includes".to_string(),
        ".jp".to_string(),
        ".jpg.html".to_string(),
        ".l".to_string(),
        ".letter".to_string(),
        ".local".to_string(),
        ".num".to_string(),
        ".pem".to_string(),
        ".php.sample".to_string(),
        ".php}".to_string(),
        ".php~".to_string(),
        ".pot".to_string(),
        ".preg-match".to_string(),
        ".process".to_string(),
        ".ps".to_string(),
        ".r".to_string(),
        ".raw".to_string(),
        ".rc".to_string(),
        ".s".to_string(),
        ".search.".to_string(),
        ".server".to_string(),
        ".sis".to_string(),
        ".sql.gz".to_string(),
        ".squery".to_string(),
        ".subscribe".to_string(),
        ".svg".to_string(),
        ".svn".to_string(),
        ".thtml".to_string(),
        ".tpl.html".to_string(),
        ".ua".to_string(),
        ".vcs".to_string(),
        ".xhtm".to_string(),
        ".xml.asp".to_string(),
        ".xpi".to_string(),
        ".A".to_string(),
        ".PAGE".to_string(),
        ".SWF".to_string(),
        ".add".to_string(),
        ".array-rand".to_string(),
        ".asax.cs".to_string(),
        ".asax.resx".to_string(),
        ".ascx.vb".to_string(),
        ".aspx,".to_string(),
        ".aspx.".to_string(),
        ".awm".to_string(),
        ".b".to_string(),
        ".bhtml".to_string(),
        ".bml".to_string(),
        ".ca".to_string(),
        ".cache".to_string(),
        ".cfg.php".to_string(),
        ".cn".to_string(),
        ".cz".to_string(),
        ".de.txt".to_string(),
        ".diff".to_string(),
        ".email".to_string(),
        ".en".to_string(),
        ".error".to_string(),
        ".faces".to_string(),
        ".filesize".to_string(),
        ".functions.php".to_string(),
        ".hml".to_string(),
        ".hqx".to_string(),
        ".html,404".to_string(),
        ".html.php".to_string(),
        ".htmls".to_string(),
        ".htx".to_string(),
        ".i".to_string(),
        ".idq".to_string(),
        ".jpe".to_string(),
        ".js.aspx".to_string(),
        ".js.gz".to_string(),
        ".jspf".to_string(),
        ".load".to_string(),
        ".media".to_string(),
        ".mp2".to_string(),
        ".mspx".to_string(),
        ".mv".to_string(),
        ".mysql".to_string(),
        ".new.php".to_string(),
        ".ocx".to_string(),
        ".oui".to_string(),
        ".outcontrol".to_string(),
        ".pad".to_string(),
        ".pages".to_string(),
        ".pdb".to_string(),
        ".pdf.".to_string(),
        ".pnp".to_string(),
        ".pop_formata_viewer".to_string(),
        ".popup.php".to_string(),
        ".popup.pop_formata_viewer".to_string(),
        ".pvk".to_string(),
        ".restrictor.log".to_string(),
        ".results".to_string(),
        ".run".to_string(),
        ".scripts".to_string(),
        ".sdb".to_string(),
        ".ser".to_string(),
        ".shop".to_string(),
        ".sitemap.xml".to_string(),
        ".smi".to_string(),
        ".start".to_string(),
        ".ste".to_string(),
        ".swf.swf".to_string(),
        ".templates".to_string(),
        ".textsearch".to_string(),
        ".torrent".to_string(),
        ".unsubscribe".to_string(),
        ".v".to_string(),
        ".vbproj.webinfo".to_string(),
        ".web".to_string(),
        ".wmf".to_string(),
        ".wpd".to_string(),
        ".ws".to_string(),
        ".xpml".to_string(),
        ".y".to_string(),
        ".AdCode".to_string(),
        ".Aspx".to_string(),
        ".C.".to_string(),
        ".COM".to_string(),
        ".GetMapImage".to_string(),
        ".Html".to_string(),
        ".Run.AdCode".to_string(),
        ".Skins".to_string(),
        ".Z".to_string(),
        ".access.login".to_string(),
        ".ajax.asp".to_string(),
        ".app".to_string(),
        ".asd".to_string(),
        ".asm".to_string(),
        ".assets".to_string(),
        ".at".to_string(),
        ".bad".to_string(),
        ".bak2".to_string(),
        ".blog".to_string(),
        ".casino".to_string(),
        ".cc".to_string(),
        ".cdr".to_string(),
        ".changeLang.php".to_string(),
        ".children".to_string(),
        ".com,".to_string(),
        ".com-redirect".to_string(),
        ".content".to_string(),
        ".copy".to_string(),
        ".count".to_string(),
        ".cp".to_string(),
        ".csproj.user".to_string(),
        ".custom".to_string(),
        ".dbf".to_string(),
        ".deb".to_string(),
        ".delete".to_string(),
        ".details.php".to_string(),
        ".dic".to_string(),
        ".divx".to_string(),
        ".download".to_string(),
        ".download.php".to_string(),
        ".downloadCirRequirements.pdf".to_string(),
        ".downloadTourkitRequirements.pdf".to_string(),
        ".emailCirRequirements.php".to_string(),
        ".emailTourkitForm.php".to_string(),
        ".emailTourkitNotification.php".to_string(),
        ".emailTourkitRequirements.php".to_string(),
        ".epub".to_string(),
        ".err".to_string(),
        ".es".to_string(),
        ".exclude".to_string(),
        ".filemtime".to_string(),
        ".fillPurposes2.php".to_string(),
        ".grp".to_string(),
        ".home".to_string(),
        ".htlm".to_string(),
        ".htm,".to_string(),
        ".html-".to_string(),
        ".image".to_string(),
        ".inc.html".to_string(),
        ".it.html".to_string(),
        ".j".to_string(),
        ".jnlp".to_string(),
        ".js.asp".to_string(),
        ".js2".to_string(),
        ".jspx".to_string(),
        ".lang-en.php".to_string(),
        ".link".to_string(),
        ".listevents".to_string(),
        ".log.0".to_string(),
        ".mbox".to_string(),
        ".mc_id".to_string(),
        ".menu.php".to_string(),
        ".mgi".to_string(),
        ".mod".to_string(),
        ".net.html".to_string(),
        ".news".to_string(),
        ".none".to_string(),
        ".off".to_string(),
        ".p3p".to_string(),
        ".php.htm".to_string(),
        ".php.static".to_string(),
        ".php1".to_string(),
        ".phpp".to_string(),
        ".pop3.php".to_string(),
        ".pop_3D_viewer".to_string(),
        ".popup.pop_3D_viewer".to_string(),
        ".prep".to_string(),
        ".prg".to_string(),
        ".print.html".to_string(),
        ".print.php".to_string(),
        ".product_details".to_string(),
        ".pwd".to_string(),
        ".pyc".to_string(),
        ".red".to_string(),
        ".registration".to_string(),
        ".requirementsFeesTable.php".to_string(),
        ".roshani-gunewardene.com".to_string(),
        ".se".to_string(),
        ".sea".to_string(),
        ".sema".to_string(),
        ".session".to_string(),
        ".setup".to_string(),
        ".simplexml-load-file".to_string(),
        ".sitx".to_string(),
        ".smil".to_string(),
        ".srv".to_string(),
        ".swi".to_string(),
        ".swp".to_string(),
        ".sxw".to_string(),
        ".tar.bz2".to_string(),
        ".tem".to_string(),
        ".temp".to_string(),
        ".template.php".to_string(),
        ".top".to_string(),
        ".txt.php".to_string(),
        ".types".to_string(),
        ".unlink".to_string(),
        ".url".to_string(),
        ".userLoginPopup.php".to_string(),
        ".visaPopup.php".to_string(),
        ".visaPopupValid.php".to_string(),
        ".vspscc".to_string(),
        ".vssscc".to_string(),
        ".w".to_string(),
        ".work".to_string(),
        ".wvx".to_string(),
        ".xspf".to_string(),
        ".-".to_string(),
        ".Admin".to_string(),
        ".E.".to_string(),
        ".Engineer".to_string(),
        ".INC".to_string(),
        ".LOG.new".to_string(),
        ".MAXIMIZE".to_string(),
        ".MPG".to_string(),
        ".NDM".to_string(),
        ".Php".to_string(),
        ".R".to_string(),
        ".SIM".to_string(),
        ".SQL".to_string(),
        ".Services".to_string(),
        ".[file".to_string(),
        ".accdb".to_string(),
        ".act".to_string(),
        ".actions.php".to_string(),
        ".admin.php".to_string(),
        ".ads".to_string(),
        ".alhtm".to_string(),
        ".all".to_string(),
        ".ani".to_string(),
        ".apf".to_string(),
        ".apj".to_string(),
        ".ar".to_string(),
        ".aral-design.com".to_string(),
        ".aral-design.de".to_string(),
        ".arc".to_string(),
        ".array-key-exists".to_string(),
        ".asp.old".to_string(),
        ".asp1".to_string(),
        ".aspg".to_string(),
        ".bfhtm".to_string(),
        ".biminifinder".to_string(),
        ".br".to_string(),
        ".browser".to_string(),
        ".build".to_string(),
        ".buscar".to_string(),
        ".categorias".to_string(),
        ".categories".to_string(),
        ".ccs".to_string(),
        ".ch".to_string(),
        ".cl".to_string(),
        ".click.php".to_string(),
        ".cls".to_string(),
        ".cls.php".to_string(),
        ".cms.ad.AdServer.cls".to_string(),
        ".com-tov.html".to_string(),
        ".com.ar".to_string(),
        ".com.br".to_string(),
        ".com.htm".to_string(),
        ".com.old".to_string(),
        ".common".to_string(),
        ".conf.php".to_string(),
        ".contact.php".to_string(),
        ".control".to_string(),
        ".core.php".to_string(),
        ".counter.php".to_string(),
        ".coverfinder".to_string(),
        ".create.php".to_string(),
        ".cs2".to_string(),
        ".d2w".to_string(),
        ".dbm".to_string(),
        ".dct".to_string(),
        ".dmb".to_string(),
        ".doc.doc".to_string(),
        ".dxf".to_string(),
        ".ed".to_string(),
        ".email.shtml".to_string(),
        ".en.htm".to_string(),
        ".engine".to_string(),
        ".env".to_string(),
        ".error-log".to_string(),
        ".esp".to_string(),
        ".ex".to_string(),
        ".exc".to_string(),
        ".exe,".to_string(),
        ".ext".to_string(),
        ".external".to_string(),
        ".ficheros".to_string(),
        ".fichiers".to_string(),
        ".flush".to_string(),
        ".fmt".to_string(),
        ".fn".to_string(),
        ".footer".to_string(),
        ".form_jhtml".to_string(),
        ".friend".to_string(),
        ".g.".to_string(),
        ".geo.xml".to_string(),
        ".ghtml".to_string(),
        ".google.com".to_string(),
        ".gov".to_string(),
        ".gpg".to_string(),
        ".hl".to_string(),
        ".href".to_string(),
        ".htm.d".to_string(),
        ".htm.html".to_string(),
        ".htm.old".to_string(),
        ".htm2".to_string(),
        ".html.orig".to_string(),
        ".html.sav".to_string(),
        ".html[".to_string(),
        ".html]".to_string(),
        ".html_".to_string(),
        ".html_files".to_string(),
        ".htmlpar".to_string(),
        ".htmlprint".to_string(),
        ".html}".to_string(),
        ".htm~".to_string(),
        ".hts".to_string(),
        ".hu".to_string(),
        ".hwp".to_string(),
        ".ibf".to_string(),
        ".il".to_string(),
        ".image.php".to_string(),
        ".imagecreatetruecolor".to_string(),
        ".imagejpeg".to_string(),
        ".iml".to_string(),
        ".imprimer".to_string(),
        ".imprimer-cadre".to_string(),
        ".imprimir".to_string(),
        ".imprimir-marco".to_string(),
        ".info.html".to_string(),
        ".info.php".to_string(),
        ".ini.bak".to_string(),
        ".ini.default".to_string(),
        ".inl".to_string(),
        ".inv".to_string(),
        ".join".to_string(),
        ".jpg.jpg".to_string(),
        ".jps".to_string(),
        ".key".to_string(),
        ".kit".to_string(),
        ".lang".to_string(),
        ".lignee".to_string(),
        ".ltr".to_string(),
        ".lzh".to_string(),
        ".m4a".to_string(),
        ".mail".to_string(),
        ".manager".to_string(),
        ".md5".to_string(),
        ".met".to_string(),
        ".metadesc".to_string(),
        ".metakeys".to_string(),
        ".mht".to_string(),
        ".min".to_string(),
        ".mld".to_string(),
        ".mobi".to_string(),
        ".mobile".to_string(),
        ".mv4".to_string(),
        ".n".to_string(),
        ".net-tov.html".to_string(),
        ".nfo".to_string(),
        ".nikon".to_string(),
        ".nodos".to_string(),
        ".nxg".to_string(),
        ".obyx".to_string(),
        ".ods".to_string(),
        ".old.2".to_string(),
        ".old.asp".to_string(),
        ".old.html".to_string(),
        ".open".to_string(),
        ".opml.config".to_string(),
        ".ord".to_string(),
        ".org.zip".to_string(),
        ".ori".to_string(),
        ".partfinder".to_string(),
        ".pho".to_string(),
        ".php-".to_string(),
        ".phpl".to_string(),
        ".phpx".to_string(),
        ".pix".to_string(),
        ".pls".to_string(),
        ".prc".to_string(),
        ".pre".to_string(),
        ".prhtm".to_string(),
        ".print-frame".to_string(),
        ".print.".to_string(),
        ".print.shtml".to_string(),
        ".printer".to_string(),
        ".properties".to_string(),
        ".propfinder".to_string(),
        ".pvx".to_string(),
        ".recherche".to_string(),
        ".redirect".to_string(),
        ".req".to_string(),
        ".roshani-gunewardene.net".to_string(),
        ".roshani-m-gunewardene.com".to_string(),
        ".safe".to_string(),
        ".sbk".to_string(),
        ".se.php".to_string(),
        ".search.asp".to_string(),
        ".sec".to_string(),
        ".seo".to_string(),
        ".serv".to_string(),
        ".server.php".to_string(),
        ".servlet".to_string(),
        ".settings".to_string(),
        ".sf".to_string(),
        ".shopping_return.php".to_string(),
        ".shopping_return_adsense.php".to_string(),
        ".show".to_string(),
        ".sht".to_string(),
        ".skins".to_string(),
        ".so".to_string(),
        ".sph".to_string(),
        ".split".to_string(),
        ".sso".to_string(),
        ".stats.php".to_string(),
        ".story".to_string(),
        ".swd".to_string(),
        ".swf.html".to_string(),
        ".sys".to_string(),
        ".tex".to_string(),
        ".tga".to_string(),
        ".thm".to_string(),
        ".tlp".to_string(),
        ".tml".to_string(),
        ".tmp.php".to_string(),
        ".touch".to_string(),
        ".tsv".to_string(),
        ".txt.".to_string(),
        ".txt.html".to_string(),
        ".ug".to_string(),
        ".unternehmen".to_string(),
        ".utf8".to_string(),
        ".vbproj.vspscc".to_string(),
        ".vsprintf".to_string(),
        ".vstemplate".to_string(),
        ".vtl".to_string(),
        ".wbmp".to_string(),
        ".webc".to_string(),
        ".webproj".to_string(),
        ".wihtm".to_string(),
        ".wp".to_string(),
        ".wps".to_string(),
        ".wri".to_string(),
        ".wsc".to_string(),
        ".www".to_string(),
        ".xsp".to_string(),
        ".xsql".to_string(),
        ".zip,".to_string(),
        ".zml".to_string(),
        ".ztml".to_string(),
        ". EXTRAHOTELERO HOSPEDAJE".to_string(),
        ". T.".to_string(),
        ". php".to_string(),
        ".,".to_string(),
        ".a5w".to_string(),
        ".aac".to_string(),
        ".access".to_string(),
        ".act.php".to_string(),
        ".action.php".to_string(),
        ".actions".to_string(),
        ".activate.php".to_string(),
        ".ad.php".to_string(),
        ".add.php".to_string(),
        ".adenaw.com".to_string(),
        ".adm".to_string(),
        ".advsearch".to_string(),
        ".ag.php".to_string(),
        ".aj_".to_string(),
        ".all.hawaii".to_string(),
        ".amaphun.com".to_string(),
        ".andriy.lviv.ua".to_string(),
        ".ap".to_string(),
        ".api".to_string(),
        ".apk".to_string(),
        ".application".to_string(),
        ".archiv".to_string(),
        ".arj".to_string(),
        ".array-map".to_string(),
        ".array-values".to_string(),
        ".art".to_string(),
        ".artdeco".to_string(),
        ".articlePk".to_string(),
        ".artnet.".to_string(),
        ".ascx.resx".to_string(),
        ".asia".to_string(),
        ".asp-".to_string(),
        ".asp.LCK".to_string(),
        ".asp.html".to_string(),
        ".asp2".to_string(),
        ".aspDONOTUSE".to_string(),
        ".asp_".to_string(),
        ".asp_files".to_string(),
        ".aspl".to_string(),
        ".aspp".to_string(),
        ".asps".to_string(),
        ".aspx.designer.cs".to_string(),
        ".aspx_files".to_string(),
        ".aspxx".to_string(),
        ".aspy".to_string(),
        ".asxp".to_string(),
        ".at.html".to_string(),
        ".avatar.php".to_string(),
        ".awstats".to_string(),
        ".babymhiasexy.com".to_string(),
        ".backup.php".to_string(),
        ".bak.php".to_string(),
        ".banan.se".to_string(),
        ".banner.php".to_string(),
        ".barnes".to_string(),
        ".basicmap.php".to_string(),
        ".baut".to_string(),
        ".bc".to_string(),
        ".best-vpn.com".to_string(),
        ".beta".to_string(),
        ".biz".to_string(),
        ".blackandmature.com".to_string(),
        ".bmp.php".to_string(),
        ".board.asd".to_string(),
        ".boom".to_string(),
        ".bossspy.org".to_string(),
        ".buscadorpornoxxx.com".to_string(),
        ".buy-here.com".to_string(),
        ".buyadspace".to_string(),
        ".bycategory".to_string(),
        ".bylocation".to_string(),
        ".bz".to_string(),
        ".c.html".to_string(),
        ".cache.inc.php".to_string(),
        ".cache.php".to_string(),
        ".car".to_string(),
        ".cascinaamalia.it".to_string(),
        ".cat.php".to_string(),
        ".catalog".to_string(),
        ".cdf".to_string(),
        ".ce".to_string(),
        ".cfm.bak".to_string(),
        ".cfsifatest.co.uk".to_string(),
        ".cfstest.co.uk".to_string(),
        ".cfswf".to_string(),
        ".cfx".to_string(),
        ".cgis".to_string(),
        ".chat".to_string(),
        ".chdir".to_string(),
        ".chloesworld.com".to_string(),
        ".classes.php".to_string(),
        ".cmp".to_string(),
        ".cnt".to_string(),
        ".co".to_string(),
        ".co-operativebank.co.uk".to_string(),
        ".co-operativebanktest.co.uk".to_string(),
        ".co-operativeinsurance.co.uk".to_string(),
        ".co-operativeinsurancetest.co.uk".to_string(),
        ".co-operativeinvestmentstest.co.uk".to_string(),
        ".co.il".to_string(),
        ".colorbox-min.js".to_string(),
        ".com-authorization-required.html".to_string(),
        ".com-bad-request.html".to_string(),
        ".com-forbidden.html".to_string(),
        ".com-internal-server-error.html".to_string(),
        ".com-page-not-found.html".to_string(),
        ".com.au".to_string(),
        ".com.php".to_string(),
        ".com.ua".to_string(),
        ".com_Backup_".to_string(),
        ".com_files".to_string(),
        ".comments".to_string(),
        ".comments.".to_string(),
        ".comments.php".to_string(),
        ".compiler.php".to_string(),
        ".conf.html".to_string(),
        ".confirm.email".to_string(),
        ".connect.php".to_string(),
        ".console".to_string(),
        ".contact".to_string(),
        ".content.php".to_string(),
        ".controller".to_string(),
        ".controls-3.1.5.swf".to_string(),
        ".cookie.js".to_string(),
        ".corp".to_string(),
        ".corp.footer".to_string(),
        ".cqs".to_string(),
        ".cron".to_string(),
        ".cropcanvas.php".to_string(),
        ".cropinterface.php".to_string(),
        ".crx".to_string(),
        ".csproj.webinfo".to_string(),
        ".csr".to_string(),
        ".css.LCK".to_string(),
        ".css.gz".to_string(),
        ".cssd".to_string(),
        ".csv.php".to_string(),
        ".ctp".to_string(),
        ".cx".to_string(),
        ".cycle.all.min.js".to_string(),
        ".d64".to_string(),
        ".daisy".to_string(),
        ".dal".to_string(),
        ".daniel".to_string(),
        ".daniel-sebald.de".to_string(),
        ".data.php".to_string(),
        ".data_".to_string(),
        ".davis".to_string(),
        ".dbml".to_string(),
        ".dcf".to_string(),
        ".de.jsp".to_string(),
        ".default.php".to_string(),
        ".del".to_string(),
        ".deleted".to_string(),
        ".dell".to_string(),
        ".demo".to_string(),
        ".desarrollo.aquihaydominios.com".to_string(),
        ".dev.bka.co.nz".to_string(),
        ".development".to_string(),
        ".dig".to_string(),
        ".display.php".to_string(),
        ".dist".to_string(),
        ".dk".to_string(),
        ".dm".to_string(),
        ".dmca-sucks.com".to_string(),
        ".dms".to_string(),
        ".dnn".to_string(),
        ".dogpl".to_string(),
        ".donothiredandobrin.com".to_string(),
        ".dontcopy".to_string(),
        ".downloadfreeporn.asia".to_string(),
        ".du".to_string(),
        ".dump".to_string(),
        ".dws".to_string(),
        ".dyn".to_string(),
        ".ea3ny.com".to_string(),
        ".easing.min.js".to_string(),
        ".ebay".to_string(),
        ".ebay.results.html".to_string(),
        ".editingoffice.com".to_string(),
        ".efacil.com.br".to_string(),
        ".ehtml".to_string(),
        ".emaximinternational.com".to_string(),
        ".en.jsp".to_string(),
        ".enn".to_string(),
        ".equonix.com".to_string(),
        ".es.html".to_string(),
        ".es.jsp".to_string(),
        ".euforyou.net".to_string(),
        ".eur".to_string(),
        ".excel.xml.php".to_string(),
        ".exec".to_string(),
        ".exp".to_string(),
        ".f.l.".to_string(),
        ".faucetdepot".to_string(),
        ".faucetdepot.com.vbproj".to_string(),
        ".faucetdepot.com.vbproj.webinfo".to_string(),
        ".fb2".to_string(),
        ".fdml".to_string(),
        ".feeds.php".to_string(),
        ".ffa".to_string(),
        ".ficken.cx".to_string(),
        ".filereader".to_string(),
        ".filters.php".to_string(),
        ".flac".to_string(),
        ".flypage".to_string(),
        ".fon".to_string(),
        ".forget.pass".to_string(),
        ".form.php".to_string(),
        ".forms".to_string(),
        ".forum".to_string(),
        ".found".to_string(),
        ".fp7".to_string(),
        ".fr.jsp".to_string(),
        ".freeasianporn.asia".to_string(),
        ".freepornxxx.asia".to_string(),
        ".frk".to_string(),
        ".frontpage.php".to_string(),
        ".ft".to_string(),
        ".ftl".to_string(),
        ".fucks.nl".to_string(),
        ".funzz.fr".to_string(),
        ".gallery.php".to_string(),
        ".garcia".to_string(),
        ".gb".to_string(),
        ".get".to_string(),
        ".get-meta-tags".to_string(),
        ".gif         ".to_string(),
        ".gif.count".to_string(),
        ".girlvandiesuburbs.co.za".to_string(),
        ".gitihost.com".to_string(),
        ".glasner.ru".to_string(),
        ".google".to_string(),
        ".gray".to_string(),
        ".gsp".to_string(),
        ".guiaweb.tk".to_string(),
        ".gutschein".to_string(),
        ".guy".to_string(),
        ".ha".to_string(),
        ".hardestlist.com".to_string(),
        ".hardpussy.com".to_string(),
        ".hasrett.de".to_string(),
        ".hawaii".to_string(),
        ".header.php".to_string(),
        ".henry".to_string(),
        ".him".to_string(),
        ".history".to_string(),
        ".hlr".to_string(),
        ".hm".to_string(),
        ".ho".to_string(),
        ".hokkaido".to_string(),
        ".hold".to_string(),
        ".home.php".to_string(),
        ".home.test".to_string(),
        ".homepage".to_string(),
        ".hp".to_string(),
        ".htm.bak".to_string(),
        ".htm.rc".to_string(),
        ".htm3".to_string(),
        ".htm5".to_string(),
        ".htm7".to_string(),
        ".htm8".to_string(),
        ".htm_".to_string(),
        ".html,,".to_string(),
        ".html-0".to_string(),
        ".html-1".to_string(),
        ".html-c".to_string(),
        ".html-old".to_string(),
        ".html-p".to_string(),
        ".html.htm".to_string(),
        ".html.images".to_string(),
        ".html.inc".to_string(),
        ".html.none".to_string(),
        ".html.pdf".to_string(),
        ".html.start".to_string(),
        ".html.txt".to_string(),
        ".html4".to_string(),
        ".html5".to_string(),
        ".html7".to_string(),
        ".htmlBAK".to_string(),
        ".htmlDolmetschen".to_string(),
        ".html_old".to_string(),
        ".htmla".to_string(),
        ".htmlc".to_string(),
        ".htmlfeed".to_string(),
        ".htmlq".to_string(),
        ".htmlu".to_string(),
        ".htn".to_string(),
        ".htpasswd".to_string(),
        ".iac.".to_string(),
        ".ibuysss.info".to_string(),
        ".iconv".to_string(),
        ".idf".to_string(),
        ".iframe_filtros".to_string(),
        ".ignore.php".to_string(),
        ".ihmtl".to_string(),
        ".ihya".to_string(),
        ".imp".to_string(),
        ".in".to_string(),
        ".inactive".to_string(),
        ".inc.php.bak".to_string(),
        ".inc.php3".to_string(),
        ".incest-porn.sex-startje.nl".to_string(),
        ".incestporn.sex-startje.nl".to_string(),
        ".incl".to_string(),
        ".indiansexzite.com".to_string(),
        ".indt".to_string(),
        ".ini.NEWCONFIGPOSSIBLYBROKEN".to_string(),
        ".insert".to_string(),
        ".internet-taxprep.com".to_string(),
        ".interpreterukraine.com".to_string(),
        ".ipl".to_string(),
        ".issues".to_string(),
        ".itml".to_string(),
        ".ixi".to_string(),
        ".jhtm".to_string(),
        ".job".to_string(),
        ".joseph".to_string(),
        ".jpf".to_string(),
        ".jpg.xml".to_string(),
        ".jpg[".to_string(),
        ".jpg]".to_string(),
        ".js,".to_string(),
        ".js.LCK".to_string(),
        ".jsa".to_string(),
        ".jsd".to_string(),
        ".jso".to_string(),
        ".jsp.old".to_string(),
        ".jsps".to_string(),
        ".jtp".to_string(),
        ".keyword".to_string(),
        ".kinkywear.net".to_string(),
        ".kk".to_string(),
        ".knvbcommunicator.voetbalassist.nl".to_string(),
        ".kokuken".to_string(),
        ".ks".to_string(),
        ".kutxa.net-en".to_string(),
        ".lang-de.php".to_string(),
        ".lang.php".to_string(),
        ".langhampartners.com".to_string(),
        ".lappgroup.com".to_string(),
        ".last".to_string(),
        ".latest".to_string(),
        ".lha".to_string(),
        ".links".to_string(),
        ".list.includes".to_string(),
        ".listMiniGrid".to_string(),
        ".listing".to_string(),
        ".lng".to_string(),
        ".loc".to_string(),
        ".local.cfm".to_string(),
        ".location.href".to_string(),
        ".log2".to_string(),
        ".lua".to_string(),
        ".lynkx".to_string(),
        ".maastrichtairporthotels.com".to_string(),
        ".mag".to_string(),
        ".mail.php".to_string(),
        ".malesextoys.us".to_string(),
        ".massivewankers.com".to_string(),
        ".mbizgroup".to_string(),
        ".mel".to_string(),
        ".members".to_string(),
        ".meretrizdelujo.com".to_string(),
        ".messagey.com".to_string(),
        ".metadata.js".to_string(),
        ".meus.php".to_string(),
        ".midi".to_string(),
        ".milliculture.net".to_string(),
        ".min_".to_string(),
        ".miss-video.com".to_string(),
        ".mk.gutschein".to_string(),
        ".mk.rabattlp".to_string(),
        ".mkv".to_string(),
        ".mmap".to_string(),
        ".model-escorts.asia".to_string(),
        ".modelescorts.asia".to_string(),
        ".mp".to_string(),
        ".mp3.html".to_string(),
        ".mq4".to_string(),
        ".mreply.rc".to_string(),
        ".msp".to_string(),
        ".mvn".to_string(),
        ".mysqli".to_string(),
        ".napravlenie_ASC".to_string(),
        ".napravlenie_DESC".to_string(),
        ".nded-pga-emial".to_string(),
        ".net-en".to_string(),
        ".net-print.htm".to_string(),
        ".net_Backup_Giornaliero".to_string(),
        ".net_Backup_Settimanale".to_string(),
        ".new.htm".to_string(),
        ".newsletter".to_string(),
        ".nexucom.com".to_string(),
        ".ninwinter.net".to_string(),
        ".nl.html".to_string(),
        ".nonude.org".to_string(),
        ".nonudes.com".to_string(),
        ".nth".to_string(),
        ".nz".to_string(),
        ".od".to_string(),
        ".offer.php".to_string(),
        ".offline".to_string(),
        ".ogv".to_string(),
        ".ok".to_string(),
        ".old.1".to_string(),
        ".old.htm".to_string(),
        ".old.old".to_string(),
        ".old1".to_string(),
        ".old3".to_string(),
        ".older".to_string(),
        ".oliver".to_string(),
        ".onedigitalcentral.com".to_string(),
        ".onenettv.com".to_string(),
        ".online".to_string(),
        ".opensearch".to_string(),
        ".org-tov.html".to_string(),
        ".org.ua-tov.html".to_string(),
        ".orig.html".to_string(),
        ".origin.php".to_string(),
        ".original.html".to_string(),
        ".orlando-vacationhome.net".to_string(),
        ".orlando-vacationhomes-pools.com".to_string(),
        ".orlando-vacationrentals.net".to_string(),
        ".osg".to_string(),
        ".outbound".to_string(),
        ".owen".to_string(),
        ".ownhometest.co.uk".to_string(),
        ".pae".to_string(),
        ".page_pls_all_password".to_string(),
        ".pages-medicales.com".to_string(),
        ".pan".to_string(),
        ".parse-url".to_string(),
        ".part".to_string(),
        ".pass".to_string(),
        ".patch".to_string(),
        ".paul".to_string(),
        ".paymethods.php".to_string(),
        ".pazderski.com".to_string(),
        ".pazderski.net".to_string(),
        ".pazderski.us".to_string(),
        ".pdd".to_string(),
        ".pdf.html".to_string(),
        ".pdf.pdf".to_string(),
        ".pdf.php".to_string(),
        ".pdfx".to_string(),
        ".perfect-color-world.com".to_string(),
        ".petersburg-apartments-for-business.html".to_string(),
        ".petersburg-apartments-for-tourists.html".to_string(),
        ".petersburg-romantic-apartments.html".to_string(),
        ".phdo".to_string(),
        ".photo".to_string(),
        ".php--------------".to_string(),
        ".php.LCK".to_string(),
        ".php.backup".to_string(),
        ".php.html".to_string(),
        ".php.inc".to_string(),
        ".php.mno".to_string(),
        ".php.original".to_string(),
        ".php_".to_string(),
        ".php_OLD".to_string(),
        ".php_old".to_string(),
        ".phphp".to_string(),
        ".phppar".to_string(),
        ".phpvreor.php".to_string(),
        ".php".to_string(),
        ".pht".to_string(),
        ".pl.html".to_string(),
        ".planetcom.ca".to_string(),
        ".playwithparis.com".to_string(),
        ".plugins".to_string(),
        ".png,bmp".to_string(),
        ".popup".to_string(),
        ".pornfailures.com".to_string(),
        ".pornoizlee.tk".to_string(),
        ".pornz.tv".to_string(),
        ".posting.prep".to_string(),
        ".prev".to_string(),
        ".print.jsp".to_string(),
        ".prl".to_string(),
        ".prosdo.com".to_string(),
        ".psb".to_string(),
        ".publisher.php".to_string(),
        ".puresolo.com".to_string(),
        ".pussyjourney.com".to_string(),
        ".qtgp".to_string(),
        ".qxd".to_string(),
        ".r.".to_string(),
        ".rabattlp".to_string(),
        ".rails".to_string(),
        ".randomocityproductions.com".to_string(),
        ".rateart.php".to_string(),
        ".readfile".to_string(),
        ".rec.html".to_string(),
        ".redirect.php".to_string(),
        ".remove".to_string(),
        ".remove.php".to_string(),
        ".removed".to_string(),
        ".resultados".to_string(),
        ".resume".to_string(),
        ".rhtm".to_string(),
        ".riddlesintime.com".to_string(),
        ".rmvb".to_string(),
        ".ro".to_string(),
        ".roma".to_string(),
        ".roomscity.com".to_string(),
        ".roshanigunewardene.com".to_string(),
        ".rpt".to_string(),
        ".rsp".to_string(),
        ".rss.php".to_string(),
        ".rss_cars".to_string(),
        ".rss_homes".to_string(),
        ".rss_jobs".to_string(),
        ".rtfd".to_string(),
        ".rvt".to_string(),
        ".s.html".to_string(),
        ".sadopasion.com".to_string(),
        ".safariextz".to_string(),
        ".salestax.php".to_string(),
        ".sc".to_string(),
        ".sca-tork.com".to_string(),
        ".scandir".to_string(),
        ".scrollTo.js".to_string(),
        ".search.html".to_string(),
        ".sec.cfm".to_string(),
        ".section".to_string(),
        ".secure".to_string(),
        ".send".to_string(),
        ".sent-".to_string(),
        ".service".to_string(),
        ".session-regenerate-id".to_string(),
        ".set".to_string(),
        ".sex-startje.nl".to_string(),
        ".sexmeme.com".to_string(),
        ".sexon.com".to_string(),
        ".sexy-girls4abo.de".to_string(),
        ".sfw".to_string(),
        ".sgf".to_string(),
        ".shipcode.php".to_string(),
        ".shipdiscount.php".to_string(),
        ".show.php".to_string(),
        ".shtml.html".to_string(),
        ".sidebar".to_string(),
        ".sisx".to_string(),
        ".sitemap.".to_string(),
        ".skin".to_string(),
        ".small-penis-humiliation.net".to_string(),
        ".smiletest.co.uk".to_string(),
        ".snippet.aspx".to_string(),
        ".snuffx.com".to_string(),
        ".sort".to_string(),
        ".sortirovka_Price.napravlenie_ASC".to_string(),
        ".sortirovka_Price.napravlenie_DESC".to_string(),
        ".sortirovka_customers_rating.napravlenie_ASC".to_string(),
        ".sortirovka_customers_rating.napravlenie_DESC".to_string(),
        ".sortirovka_name.napravlenie_ASC".to_string(),
        ".sortirovka_name.napravlenie_DESC".to_string(),
        ".sp".to_string(),
        ".sphp3".to_string(),
        ".srch".to_string(),
        ".srf".to_string(),
        ".srvl".to_string(),
        ".st-patricks.com".to_string(),
        ".sta".to_string(),
        ".staged.php".to_string(),
        ".staging".to_string(),
        ".start.php".to_string(),
        ".stat".to_string(),
        ".stats".to_string(),
        ".step".to_string(),
        ".stml".to_string(),
        ".storebanner.php".to_string(),
        ".storelogo.php".to_string(),
        ".storename.php".to_string(),
        ".sts.php".to_string(),
        ".suarez".to_string(),
        ".submit".to_string(),
        ".support".to_string(),
        ".support.html".to_string(),
        ".swf.LCK".to_string(),
        ".sym".to_string(),
        ".system".to_string(),
        ".tab-".to_string(),
        ".table.html".to_string(),
        ".tablesorter.min.js".to_string(),
        ".tablesorter.pager.js".to_string(),
        ".tatianyc.com".to_string(),
        ".tb".to_string(),
        ".tech".to_string(),
        ".teen-shy.com".to_string(),
        ".teenhardpussy.com".to_string(),
        ".temp.php".to_string(),
        ".templates.php".to_string(),
        ".temporarily.withdrawn.html".to_string(),
        ".test.cgi".to_string(),
        ".test.php".to_string(),
        ".tf".to_string(),
        ".tg".to_string(),
        ".thanks".to_string(),
        ".thehotfish.com".to_string(),
        ".theme".to_string(),
        ".thompson".to_string(),
        ".thumb.jpg".to_string(),
        ".ticket.submit".to_string(),
        ".tim".to_string(),
        ".tk".to_string(),
        ".tls".to_string(),
        ".to".to_string(),
        ".touch.action".to_string(),
        ".trace".to_string(),
        ".tracker.ashx".to_string(),
        ".trade".to_string(),
        ".trishasex.viedos.com".to_string(),
        ".ts".to_string(),
        ".tst".to_string(),
        ".tvpi".to_string(),
        ".txt.txt".to_string(),
        ".txuri-urdin.com".to_string(),
        ".ufo".to_string(),
        ".ugmart.ug".to_string(),
        ".ui-1.5.2".to_string(),
        ".unixteacher.org".to_string(),
        ".unsharp.php".to_string(),
        ".update".to_string(),
        ".upgrade".to_string(),
        ".v1.11.js".to_string(),
        ".v2.php".to_string(),
        ".vacationhomes-pools.com".to_string(),
        ".var".to_string(),
        ".venetian.com,prod2.venetian.com,reservations.venetian.com,".to_string(),
        ".verify".to_string(),
        ".video".to_string(),
        ".videodeputas.com".to_string(),
        ".videos-chaudes.com".to_string(),
        ".viewpage__10".to_string(),
        ".vmdk".to_string(),
        ".vn".to_string(),
        ".voetbalassist.nl".to_string(),
        ".vs".to_string(),
        ".vx".to_string(),
        ".vxlpub".to_string(),
        ".w3m".to_string(),
        ".w3x".to_string(),
        ".wax".to_string(),
        ".web-teck.com".to_string(),
        ".webalizer".to_string(),
        ".webarchive".to_string(),
        ".webjockey.nl".to_string(),
        ".webm".to_string(),
        ".weedooz.eu".to_string(),
        ".wgx".to_string(),
        ".wimzi.php".to_string(),
        ".wireless".to_string(),
        ".wireless.action".to_string(),
        ".wm".to_string(),
        ".woolovers.com".to_string(),
        ".working".to_string(),
        ".wpl".to_string(),
        ".wplus".to_string(),
        ".wps.rtf".to_string(),
        ".write.php".to_string(),
        ".wwsec_app_priv.login".to_string(),
        ".www.annuaire-vimarty.net".to_string(),
        ".www.annuaire-web.info".to_string(),
        ".www.kit-graphik.com".to_string(),
        ".www.photo-scope.fr".to_string(),
        ".xcam.at".to_string(),
        ".xconf".to_string(),
        ".xcwc.com".to_string(),
        ".xgi".to_string(),
        ".xhtml5".to_string(),
        ".xlt".to_string(),
        ".xm".to_string(),
        ".xml.old".to_string(),
        ".xpdf".to_string(),
        ".xqy".to_string(),
        ".xslx".to_string(),
        ".xst".to_string(),
        ".xsx".to_string(),
        ".xy.php".to_string(),
        ".yp".to_string(),
        ".ys".to_string(),
        ".z".to_string(),
        ".za".to_string(),
        ".zh.html".to_string(),
        ".zhtml".to_string(),
        ".zip.php".to_string(),
    ];
    valid_extension.contains(&tested_extension.to_string())
}

pub fn remove_extension(file_name: &str) -> String {
    let extension = get_extension(file_name);
    if extension.is_none() {
        return file_name.to_owned();
    }

    let regex_string = format!(r"\.{}$", extension.unwrap());
    let extension_regex = Regex::new(&regex_string).unwrap();
    let removed = extension_regex.replace_all(file_name, "");

    format!("{}", removed)
}

pub fn get_extension(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let p_extension = path.extension()?;
    let string_extension = format!(".{}", p_extension.to_str().unwrap());
    if !is_valid_extension(&string_extension) {
        return None;
    }
    Some(p_extension.to_str().unwrap().to_string())
}

#[cfg(test)]
mod test {
    use crate::parsing::extensions::{get_extension, is_valid_extension, remove_extension};

    #[test]
    fn test_valid_extension() {
        assert!(is_valid_extension(".mkv"));
        assert!(is_valid_extension(".mp4"));
        assert!(is_valid_extension(".avi"));
        assert!(is_valid_extension(".html"));
    }

    #[test]
    fn test_invalid_extension() {
        assert!(!is_valid_extension("mkv"));
        assert!(!is_valid_extension(".non an extension"));
        assert!(!is_valid_extension("avi"));
    }

    #[test]
    fn get_extension_mkv() {
        assert_eq!(get_extension("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension("[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Eve no Jikan 2 [88F4F7F0].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(get_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Detective Conan - 316-317 [DCTP][2411959B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[N LogN Fansubs] Angel Beats (9).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("After War Gundam X - 1x03 - My Mount is Fierce!.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Red Data Girl - 10v0 [29EA865B].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv").unwrap(),"mkv");
        assert_eq!(get_extension("[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv"
        )
        .unwrap(),
        "mkv"
    );
        assert_eq!(
            get_extension(
                "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Mary Bell (DVD) - 01v2 [h-b].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Mary Bell (DVD) - 02 [h-b].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("The Irregular at Magic High School - S01E01- Enrollment Part I.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Ruri]No.6 01 [720p][H264][A956075C].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("01 - Land of Visible Pain.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("The iDOLM@STER 765 Pro to Iu Monogatari.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[HorribleSubs] Gintama - 111C [1080p].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension("[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv").unwrap(),"mkv");
        assert_eq!(get_extension("[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv").unwrap(),"mkv");
        assert_eq!(
            get_extension(
                "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(get_extension("[TardRaws] 0 [640x360].mkv").unwrap(), "mkv");
        assert_eq!(
            get_extension("[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv")
                .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension(
                "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv"
            )
            .unwrap(),
            "mkv"
        );
        assert_eq!(
            get_extension("[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv").unwrap(),
            "mkv"
        );
        assert_eq!(
        get_extension(
            "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv"
        )
        .unwrap(),
        "mkv"
    );
    }

    #[test]
    fn get_extension_avi() {
        assert_eq!(
            get_extension("[DB]_Bleach_225_[C63D149C].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi")
                .unwrap(),
            "avi"
        );
        assert_eq!(get_extension("Juuni.Kokki.Ep.5.avi").unwrap(), "avi");
        assert_eq!(get_extension("Juuni Kokki Ep.5.avi").unwrap(), "avi");
        assert_eq!(
            get_extension("[Keroro].148.[Xvid.mp3].[FE68D5F1].avi").unwrap(),
            "avi"
        );
        assert_eq!(
            get_extension("[Triad]_Today_in_Class_5-2_-_04.avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("__BLUE DROP 10 (1).avi").unwrap(), "avi");
        assert_eq!(
            get_extension("37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi").unwrap(),"avi");
        assert_eq!(
            get_extension("[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi").unwrap(),
            "avi"
        );
        assert_eq!(get_extension("[KLF]_D.Gray-man_04V2.avi").unwrap(), "avi");
    }

    #[test]
    fn get_extension_mp4() {
        assert_eq!(
            get_extension("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4").unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension("「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension(
                "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4").unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension(
                "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(get_extension("[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4").unwrap(),"mp4");
        assert_eq!(
            get_extension(
                "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4"
            )
            .unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4")
                .unwrap(),
            "mp4"
        );
        assert_eq!(
        get_extension(
            "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4"
        )
        .unwrap(),
        "mp4"
    );
        assert_eq!(
            get_extension("Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4").unwrap(),
            "mp4"
        );
        assert_eq!(
            get_extension("[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4").unwrap(),
            "mp4"
        );
    }

    #[test]
    fn get_extension_none() {
        assert!(get_extension("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]").is_none());
        assert!(
            get_extension("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]").is_none()
        );
        assert!(
            get_extension("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3").is_none()
        );
        assert!(get_extension("Chrono Crusade ep. 1-5").is_none());
        assert!(get_extension("[SFW]_Queen's_Blade_S2").is_none());
        assert!(get_extension("Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]").is_none());
        assert!(get_extension(
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
        )
        .is_none());
        assert!(get_extension(
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
        )
        .is_none());
        assert!(get_extension("【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]").is_none());
        assert!(get_extension("Vol.01").is_none());
        assert!(get_extension(
            "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
        )
        .is_none());
        assert!(get_extension("DRAMAtical Murder Episode 1 - Data_01_Login").is_none());
        assert!(get_extension("[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)").is_none());
        assert!(get_extension(
            "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
        )
        .is_none());
        assert!(get_extension(
            "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
        )
        .is_none());
        assert!(get_extension("Ep. 01 - The Boy in the Iceberg").is_none());
        assert!(get_extension(
            "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"
        )
        .is_none());
        assert!(get_extension("[LRL] 1001 Nights (1998) [DVD]").is_none());
        assert!(get_extension("[Anime").is_none());
        assert!(get_extension("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)").is_none());
        assert!(get_extension(
            "[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]"
        )
        .is_none());
        assert!(get_extension("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]").is_none());
        assert!(
            get_extension("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)")
                .is_none()
        );
        assert!(get_extension(
            "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
        )
        .is_none());
        assert!(
            get_extension("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]")
                .is_none()
        );
        assert!(get_extension(
            "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
        )
        .is_none());
        assert!(get_extension(
            "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
        )
        .is_none());
        assert!(get_extension("Noragami - S02E06 - What Must Be Done [Episode 6]").is_none());
        assert!(get_extension("[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]").is_none());
        assert!(get_extension("[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)").is_none());
        assert!(get_extension("[Infantjedi] Norn9 - Norn + Nonetto - 12").is_none());
        assert!(
            get_extension("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA").is_none()
        );
        assert!(get_extension("[HorribleSubs] Momokuri - 01+02 [720p]").is_none());
        assert!(get_extension("").is_none());
    }

    #[test]
    fn get_other_extension() {
        assert_eq!(
            get_extension("[Hard-Boiled FS]FullMetalAlchemist_09.rmvb").unwrap(),
            "rmvb"
        );
        assert_eq!(
        get_extension(
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z"
        )
        .unwrap(),
        "7z"
    );
        assert_eq!(
        get_extension(
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch.7z"
        )
        .unwrap(),
        "7z"
    );
    }

    #[test]
    fn mkv() {
        assert_eq!(remove_extension("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv"),"[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]");
        assert_eq!(
            remove_extension("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv"),
            "[ANBU]_Princess_Lover!_-_01_[2048A39A]"
        );
        assert_eq!(
            remove_extension("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv"),
            "[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89]"
        );
        assert_eq!(
            remove_extension("[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6].mkv"),
            "[ANBU-umai]_Haiyoru!_Nyaru-Ani_[596DD8E6]"
        );
        assert_eq!(
            remove_extension("[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9].mkv"),
            "[BakaWolf-m.3.3.w] Special A 01 (H.264) [C83164B9]"
        );
        assert_eq!(
            remove_extension(
                "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090].mkv"
            ),
            "[chibi-Doki] Seikon no Qwaser - 13v0 (Uncensored Director's Cut) [988DB090]"
        );
        assert_eq!(
            remove_extension(
                "[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv"
            ),
            "[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B]"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6].mkv"),
            "[Coalgirls]_Toradora_ED2_(704x480_DVD_AAC)_[3B65D1E6]"
        );
        assert_eq!(
            remove_extension(
                "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8].mkv"
            ),
            "[Conclave-Mendoi]_Mobile_Suit_Gundam_00_S2_-_01v2_[1280x720_H.264_AAC][4863FBE8]"
        );
        assert_eq!(
            remove_extension("[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735].mkv"),
            "[Frostii]_Nodame_Cantabile_Finale_-_00_[73AD0735]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv"),
            "[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p]"
        );
        assert_eq!(
            remove_extension(
                "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF].mkv"
            ),
            "[Juuni.Kokki]-(Les.12.Royaumes)-[Ep.24]-[x264+OGG]-[JAP+FR+Sub.FR]-[Chap]-[AzF]"
        );
        assert_eq!(
            remove_extension("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv"),
            "[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}]"
        );
        assert_eq!(
            remove_extension(
                "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD].mkv"
            ),
            "[Lambda-Delta]_Umineko_no_Naku_Koro_ni_-_11_[848x480_H.264_AAC][943106AD]"
        );
        assert_eq!(
            remove_extension("[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F].mkv"),
            "[SS]_Kemono_no_Souja_Erin_-_12_(1280x720_h264)_[0F5F884F]"
        );
        assert_eq!(
            remove_extension("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv"),
            "[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB]"
        );
        assert_eq!(
            remove_extension(
                "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub].mkv"
            ),
            "[ValdikSS]_First_Squad_The_Morment_Of_Truth_[720x576_h264_dvdscr_eng_hardsub]"
        );
        assert_eq!(
            remove_extension(
                "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU].mkv"
            ),
            "Code_Geass_R2_TV_[20_of_25]_[ru_jp]_[HDTV]_[Varies_&_Cuba77_&_AnimeReactor_RU]"
        );
        assert_eq!(
            remove_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            ),
            "Evangelion_1.11_You_Are_(Not)_Alone_(2009)_[1080p,BluRay,x264,DTS-ES]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv"
            ),
            "Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA"
        );
        assert_eq!(
            remove_extension("Eve no Jikan 2 [88F4F7F0].mkv"),
            "Eve no Jikan 2 [88F4F7F0]"
        );
        assert_eq!(
            remove_extension("Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA.mkv"),
            "Gin'iro_no_Kami_no_Agito_(2006)_[1080p,BluRay,x264,DTS]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv"
            ),
            "Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769]"
        );
        assert_eq!(remove_extension("Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA.mkv"),"Mobile_Suit_Gundam_00_Season_2_Ep07_A_Reunion_and_a_Parting_[1080p,BluRay,x264]_-_THORA");
        assert_eq!(
            remove_extension("Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru].mkv"),
            "Noein_[01_of_24]_[ru_jp]_[bodlerov_&_torrents_ru]"
        );
        assert_eq!(
            remove_extension("ponyo_on_the_cliff_by_the_sea[h264.dts][niizk].mkv"),
            "ponyo_on_the_cliff_by_the_sea[h264.dts][niizk]"
        );
        assert_eq!(
            remove_extension(
                "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv"
            ),
            "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A]"
        );
        assert_eq!(
            remove_extension("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv"),
            "[a4e]R.O.D_the_TV_01[divx5.2.1]"
        );
        assert_eq!(remove_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"),"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA");
        assert_eq!(remove_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"),"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA");
        assert_eq!(
            remove_extension(
                "Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv"
            ),
            "Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971].mkv"),
            "[RaX]Mezzo(DSA)_-_05_-_[x264_ogg]_[585d9971]"
        );
        assert_eq!(
            remove_extension("[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B].mkv"),
            "[AKH-SWE]_Fullmetal_Alchemist_(2009)_02v2_[H.264.AAC][7B2C5E8B]"
        );
        assert_eq!(
            remove_extension("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv"),
            "[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3]"
        );
        assert_eq!(
            remove_extension("[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B].mkv"),
            "[Ayu]_Kiddy_Grade_2_-_Pilot_[H264_AC3][650B731B]"
        );
        assert_eq!(
            remove_extension(
                "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3].mkv"
            ),
            "[Darksoul-Subs] Tatakau Shisho - The Book of Bantorra [848x480 XVID_MP3]"
        );
        assert_eq!(remove_extension("[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577].mkv"),"[ACX]Neon_Genesis_Evangelion_-_Platinum_-_06_-_Showdown_in_Tokyo_3_[SaintDeath]_[CBDB8577]");
        assert_eq!(
            remove_extension("[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672.mkv"),
            "[Himatsubushi]_Sora_no_Woto_-_01_-_H264_-_720p_-_E83AD672"
        );
        assert_eq!(
            remove_extension(
                "[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv"
            ),
            "[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314]"
        );
        assert_eq!(
            remove_extension("Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253].mkv"),
            "Ookiku Furikabutte S2 - 09 (Central Anime) [BD841253]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi).mkv"),
            "[HorribleSubs] HEROMAN - 10_(XviD_AnimeSenshi)"
        );
        assert_eq!(
            remove_extension("Detective Conan - 316-317 [DCTP][2411959B].mkv"),
            "Detective Conan - 316-317 [DCTP][2411959B]"
        );
        assert_eq!(
            remove_extension("[N LogN Fansubs] Angel Beats (9).mkv"),
            "[N LogN Fansubs] Angel Beats (9)"
        );
        assert_eq!(
            remove_extension("Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487].mkv"),
            "Juuousei_-_01_[Black_Sheep][HDTV_H264_AAC][803DA487]"
        );
        assert_eq!(
            remove_extension("[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4].mkv"),
            "[gg]_Kimi_ni_Todoke_2nd_Season_-_00_[BF735BC4]"
        );
        assert_eq!(
            remove_extension("K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA.mkv"),
            "K-ON!_Ep03_Training!_[1080p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA.mkv"),
            "K-ON!!_Ep08_Career_Plan!_[1080p,BluRay,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension(
                "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991].mkv"
            ),
            "[E-HARO Raws] Kore wa Zombie desu ka - 03 (TV 1280x720 h264 AAC) [888E4991]"
        );
        assert_eq!(
            remove_extension("[Edomae Subs] Kore wa Zombie desu ka  Episode 2.mkv"),
            "[Edomae Subs] Kore wa Zombie desu ka  Episode 2"
        );
        assert_eq!(
            remove_extension("5_centimeters_per_second[1904x1072.h264.flac][niizk].mkv"),
            "5_centimeters_per_second[1904x1072.h264.flac][niizk]"
        );
        assert_eq!(
            remove_extension("[Yoroshiku]_009-1_-_02_(H264)_[36D2444D].mkv"),
            "[Yoroshiku]_009-1_-_02_(H264)_[36D2444D]"
        );
        assert_eq!(
            remove_extension("After War Gundam X - 1x03 - My Mount is Fierce!.mkv"),
            "After War Gundam X - 1x03 - My Mount is Fierce!"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] The World God Only Knows 2 - 03 [720p].mkv"),
            "[HorribleSubs] The World God Only Knows 2 - 03 [720p]"
        );
        assert_eq!(
            remove_extension(
                "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25].mkv"
            ),
            "Macross Frontier - Sayonara no Tsubasa (Central Anime, 720p) [46B35E25]"
        );
        assert_eq!(
            remove_extension("[FFF] Red Data Girl - 10v0 [29EA865B].mkv"),
            "[FFF] Red Data Girl - 10v0 [29EA865B]"
        );
        assert_eq!(
            remove_extension("[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685].mkv"),
            "[CMS] Magical☆Star Kanon 100% OVA[DVD][E9F43685]"
        );
        assert_eq!(
            remove_extension("[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D].mkv"),
            "[Doremi].Ro-Kyu-Bu!.SS.01.[C1B5CE5D]"
        );
        assert_eq!(remove_extension("[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B].mkv"),"[Raizel] Persona 4 The Animation Episode 13 - A Stormy Summer Vacation Part 1  [BD_1080p_Dual_Audio_FLAC_Hi10p][8A45634B]");
        assert_eq!(remove_extension("[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015].mkv"),"[Hien] Kotoura-san - Special Short Anime 'Haruka's Room' - 01 [BD 1080p H.264 10-bit AAC][6B6BE015]");
        assert_eq!(
            remove_extension("[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36].mkv"),
            "[R-R] Diebuster.EP1 (720p.Hi10p.AC3) [82E36A36]"
        );
        assert_eq!(
            remove_extension(
                "Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv"
            ),
            "Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF]"
        );
        assert_eq!(
            remove_extension("[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis.mkv"),
            "[Rakuda].Gift.~eternal.rainbow~.01.dvd.h.264.vorbis"
        );
        assert_eq!(
            remove_extension(
                "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv"
            ),
            "[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7)"
        );
        assert_eq!(
            remove_extension("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv"),
            "[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873]"
        );
        assert_eq!(
            remove_extension("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv"),
            "[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530]"
        );
        assert_eq!(
            remove_extension("[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D].mkv"),
            "[UTW]_Fate_Zero_-_01_[BD][h264-720p_AC3][02A0491D]"
        );
        assert_eq!(
            remove_extension(
                "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5].mkv"
            ),
            "[UTW-THORA] Evangelion 3.33 You Can (Not) Redo [BD][1080p,x264,flac][F2060CF5]"
        );
        assert_eq!(
            remove_extension(
                "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv"
            ),
            "Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank"
        );
        assert_eq!(
            remove_extension(
                "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv"
            ),
            "Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2"
        );
        assert_eq!(
            remove_extension(
                "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv"
            ),
            "[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76]"
        );
        assert_eq!(
            remove_extension("[Asenshi] Rozen Maiden 3 - PV [CA57F300].mkv"),
            "[Asenshi] Rozen Maiden 3 - PV [CA57F300]"
        );
        assert_eq!(
            remove_extension("Mary Bell (DVD) - 01v2 [h-b].mkv"),
            "Mary Bell (DVD) - 01v2 [h-b]"
        );
        assert_eq!(
            remove_extension("Mary Bell (DVD) - 02 [h-b].mkv"),
            "Mary Bell (DVD) - 02 [h-b]"
        );
        assert_eq!(
            remove_extension("The Irregular at Magic High School - S01E01- Enrollment Part I.mkv"),
            "The Irregular at Magic High School - S01E01- Enrollment Part I"
        );
        assert_eq!(
            remove_extension("[Mezashite] Aikatsu! ‒ 100 [D035A39F].mkv"),
            "[Mezashite] Aikatsu! ‒ 100 [D035A39F]"
        );
        assert_eq!(
            remove_extension("[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv"),
            "[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18]"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Tsukimonogatari - (01-04) [1080p].mkv"),
            "[HorribleSubs] Tsukimonogatari - (01-04) [1080p]"
        );
        assert_eq!(
            remove_extension("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv"),
            "[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed]"
        );
        assert_eq!(
            remove_extension(
                "[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv"
            ),
            "[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375]"
        );
        assert_eq!(
            remove_extension("[Ruri]No.6 01 [720p][H264][A956075C].mkv"),
            "[Ruri]No.6 01 [720p][H264][A956075C]"
        );
        assert_eq!(
            remove_extension("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv"),
            "EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2]"
        );
        assert_eq!(
            remove_extension("01 - Land of Visible Pain.mkv"),
            "01 - Land of Visible Pain"
        );
        assert_eq!(remove_extension("[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED].mkv"),"[Zom-B] Machine-Doll wa Kizutsukanai - 01 - Facing ''Cannibal Candy'' I (kuroi, FFF remux) [B99C8DED]");
        assert_eq!(
            remove_extension("The iDOLM@STER 765 Pro to Iu Monogatari.mkv"),
            "The iDOLM@STER 765 Pro to Iu Monogatari"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026].mkv"),
            "[Coalgirls]_Fate_Zero_OVA3.5_(1280x720_Blu-ray_FLAC)_[5F5AD026]"
        );
        assert_eq!(
            remove_extension("[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0).mkv"),
            "[GrimRipper] Koi Kaze [Dual Audio] Ep01 (c13cefe0)"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Gintama - 111C [1080p].mkv"),
            "[HorribleSubs] Gintama - 111C [1080p]"
        );
        assert_eq!(
            remove_extension("[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458].mkv"),
            "[SpoonSubs]_Hidamari_Sketch_x365_-_04.1_(DVD)[B6CE8458]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv"
            ),
            "[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0]"
        );
        assert_eq!(
            remove_extension("[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8].mkv"),
            "[Elysium]Sora.no.Woto.EP07.5(BD.720p.AAC)[C37580F8]"
        );
        assert_eq!(remove_extension("[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7].mkv"),"[Zurako] Sora no Woto - 07.5 - Drinking Party - Fortress Battle (BD 1080p AAC) [F7DF16F7]");
        assert_eq!(
            remove_extension("[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720].mkv"),
            "[Hiryuu] Maji de Watashi ni Koi Shinasai!! - 02 [720]"
        );
        assert_eq!(
            remove_extension(
                "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355].mkv"
            ),
            "[Kira-Fansub] Uchuu no Stellvia ep 14 (BD H264 1280x960 24fps AAC) [06EE7355]"
        );
        assert_eq!(
            remove_extension(
                "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC].mkv"
            ),
            "[ANE] Yosuga no Sora - Ep01 Preview (Yorihime ver) [BDRip 1080p x264 FLAC]"
        );
        assert_eq!(
            remove_extension("[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1].mkv"),
            "[B-G_&_m.3.3.w]_Myself_Yourself_12.DVD(H.264_DD2.0)_[CB2B37F1]"
        );
        assert_eq!(
            remove_extension("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv"),
            "The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR"
        );
        assert_eq!(remove_extension("[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv"),"[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6]");
        assert_eq!(remove_extension("[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv"),"[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F]");
        assert_eq!(
            remove_extension(
                "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv"
            ),
            "Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145)"
        );
        assert_eq!(
            remove_extension("[ReDone] Memories Off 3.5 - 04 (DVD 10-bit).mkv"),
            "[ReDone] Memories Off 3.5 - 04 (DVD 10-bit)"
        );
        assert_eq!(
            remove_extension(
                "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv"
            ),
            "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv"
            ),
            "[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7]"
        );
        assert_eq!(
            remove_extension("[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B].mkv"),
            "[Deep] Tegami Bachi (REVERSE) - Letter Bee - 29 (04) [5203142B]"
        );
        assert_eq!(
            remove_extension("[FFF] Love Live! The School Idol Movie - PV [D1A15D2C].mkv"),
            "[FFF] Love Live! The School Idol Movie - PV [D1A15D2C]"
        );
        assert_eq!(
            remove_extension(
                "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv"
            ),
            "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607]"
        );
        assert_eq!(
            remove_extension("[TardRaws] 0 [640x360].mkv"),
            "[TardRaws] 0 [640x360]"
        );
        assert_eq!(
            remove_extension(
                "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv"
            ),
            "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5]"
        );
        assert_eq!(
            remove_extension(
                "[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv"
            ),
            "[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066]"
        );
        assert_eq!(
            remove_extension("[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745].mkv"),
            "[NamaeNai] Hidamari Sketch x365 - 09a (DVD) [49874745]"
        );
        assert_eq!(
            remove_extension("[FaggotryRaws] Bakuman - 01 (NHK E 848x480).mkv"),
            "[FaggotryRaws] Bakuman - 01 (NHK E 848x480)"
        );
        assert_eq!(
            remove_extension("Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD.mkv"),
            "Dragon.Ball.KAI.-.01.-.1080p.BluRay.x264.DHD"
        );
        assert_eq!(
            remove_extension("[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent].mkv"),
            "[AnimeRG] Ushio to Tora (TV) - 02 [720p] [Xcelent]"
        );
        assert_eq!(
            remove_extension(
                "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja.mkv"
            ),
            "[(´• ω •`)] Nintama Rantarou - S23E1821 - Buddhist Priest-sama is a Ninja"
        );
        assert_eq!(
            remove_extension("[Judas] Aharen-san wa Hakarenai - S01E06v2.mkv"),
            "[Judas] Aharen-san wa Hakarenai - S01E06v2"
        );
        assert_eq!(
            remove_extension(
                "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531].mkv"
            ),
            "[0x539] Somali and the Forest Spirit - S01E01 (WEB 1080p Hi10P AAC) [BB7C6531]"
        );
    }

    #[test]
    fn avi() {
        assert_eq!(
            remove_extension("[DB]_Bleach_225_[C63D149C].avi"),
            "[DB]_Bleach_225_[C63D149C]"
        );
        assert_eq!(
            remove_extension("[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD.avi"),
            "[KAF-TEAM]_One_Piece_Movie_9_vostfr_HD"
        );
        assert_eq!(
            remove_extension("[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378].avi"),
            "[RNA]_Sakura_Taisen_New_York_NY_Ep_2_[1590D378]"
        );
        assert_eq!(
            remove_extension("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi"),
            "[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71]"
        );
        assert_eq!(remove_extension("Juuni.Kokki.Ep.5.avi"), "Juuni.Kokki.Ep.5");
        assert_eq!(remove_extension("Juuni Kokki Ep.5.avi"), "Juuni Kokki Ep.5");
        assert_eq!(
            remove_extension("[Keroro].148.[Xvid.mp3].[FE68D5F1].avi"),
            "[Keroro].148.[Xvid.mp3].[FE68D5F1]"
        );
        assert_eq!(
            remove_extension("[Triad]_Today_in_Class_5-2_-_04.avi"),
            "[Triad]_Today_in_Class_5-2_-_04"
        );
        assert_eq!(
            remove_extension("__BLUE DROP 10 (1).avi"),
            "__BLUE DROP 10 (1)"
        );
        assert_eq!(
            remove_extension("37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273].avi"),
            "37 [Ruberia]_Death_Note_-_37v2_[FINAL]_[XviD][6FA7D273]"
        );
        assert_eq!(remove_extension("[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2.avi"),"[FB] Crayon Shin-Chan Movie 2 The Secret of Buri Buri Kingdom [DivX5 AC3] 1994 [852X480] V2");
        assert_eq!(
            remove_extension("[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315].avi"),
            "[Shinsen-Subs]_Macross_Frontier_-_01b_[4D5EC315]"
        );
        assert_eq!(
            remove_extension("[KLF]_D.Gray-man_04V2.avi"),
            "[KLF]_D.Gray-man_04V2"
        );
    }

    #[test]
    fn mp4() {
        assert_eq!(
            remove_extension("[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957].mp4"),
            "[Taka]_Fullmetal_Alchemist_(2009)_04_[720p][40F2A957]"
        );
        assert_eq!(remove_extension("[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED].mp4"),"[Mobile Suit Gundam Seed Destiny HD REMASTER][07][Big5][720p][AVC_AAC][encoded by SEED]");
        assert_eq!(
            remove_extension("「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC).mp4"),
            "「K」 Image Blu-ray WHITE & BLACK - Main (BD 1280x720 AVC AAC)"
        );
        assert_eq!(
            remove_extension("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4"),
            "[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720].mp4"
            ),
            "[Hakugetsu&Speed&MGRT][Dragon_Ball_Z_Battle_of_Gods][BDRIP][BIG5][1280x720]"
        );
        assert_eq!(
            remove_extension("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4"),
            "[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0]"
        );
        assert_eq!(remove_extension("[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap].mp4"),"[TV-J] Kidou Senshi Gundam UC Unicorn - episode.02 [BD 1920x1080 h264+AAC(5.1ch JP+EN) +Sub(JP-EN-SP-FR-CH) Chap]");
        assert_eq!(
            remove_extension("[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC).mp4"),
            "[Zero-Raws] Shingeki no Kyojin - 25 END (MBS 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension("Bakemonogatari - 01 (BD 1280x720 AVC AACx2).mp4"),
            "Bakemonogatari - 01 (BD 1280x720 AVC AACx2)"
        );
        assert_eq!(
            remove_extension("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4"),
            "Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0])"
        );
        assert_eq!(
            remove_extension("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4"),
            "Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3)"
        );
        assert_eq!(
            remove_extension(
                "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4"
            ),
            "Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC)"
        );
        assert_eq!(
            remove_extension("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4"),
            "[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体]"
        );
        assert_eq!(remove_extension("[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4"),"[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P]");
        assert_eq!(
            remove_extension(
                "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4"
            ),
            "[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4"
            ),
            "[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6].mp4"
            ),
            "[SubDESU-H] Swing out Sisters Complete Version (720p x264 8bit AC3) [3ABD57E6]"
        );
        assert_eq!(
            remove_extension("Cyborg 009 (1968) [TSHS] episode 06 [30C15D62].mp4"),
            "Cyborg 009 (1968) [TSHS] episode 06 [30C15D62]"
        );
        assert_eq!(
            remove_extension("[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4"),
            "[5F] RWBY 14 Forever Fall Part 2 pt-BR"
        );
    }

    #[test]
    fn no_extension() {
        assert_eq!(
            remove_extension("To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]"),
            "To_Aru_Kagaku_no_Railgun_13-15_[BD_1080p][AtsA]"
        );
        assert_eq!(
            remove_extension("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]"),
            "Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]"
        );
        assert_eq!(
            remove_extension("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3"),
            "[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3"
        );
        assert_eq!(
            remove_extension("Chrono Crusade ep. 1-5"),
            "Chrono Crusade ep. 1-5"
        );
        assert_eq!(
            remove_extension("[SFW]_Queen's_Blade_S2"),
            "[SFW]_Queen's_Blade_S2"
        );
        assert_eq!(
            remove_extension("Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]"),
            "Evangelion_1.0_You_Are_[Not]_Alone_(1080p)_[@Home]"
        );
        assert_eq!(
            remove_extension(
                "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
            ),
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 8 bit AAC)[BA70BA9C]"
        );
        assert_eq!(
            remove_extension(
                "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
            ),
            "[Nubles] Space Battleship Yamato 2199 (2012) episode 18 (720p 10 bit AAC)[1F56D642]"
        );
        assert_eq!(
            remove_extension("【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]"),
            "【MMZYSUB】★【Golden Time】[24（END）][GB][720P_MP4]"
        );
        assert_eq!(remove_extension("Vol.01"), "Vol.01");
        assert_eq!(
            remove_extension(
                "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
            ),
            "Attack on Titan - Episode 3 - A Dim Light Amid Despair / Humanity's Comeback, Part 1"
        );
        assert_eq!(
            remove_extension("DRAMAtical Murder Episode 1 - Data_01_Login"),
            "DRAMAtical Murder Episode 1 - Data_01_Login"
        );
        assert_eq!(
            remove_extension("[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)"),
            "[Coalgirls]_White_Album_1-13_(1280×720_Blu-Ray_FLAC)"
        );
        assert_eq!(
            remove_extension(
                "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
            ),
            "[CH] Sword Art Online Extra Edition Dual Audio [BD 480p][10bitH.264+Vorbis]"
        );
        assert_eq!(
            remove_extension(
                "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
            ),
            "Episode 14 Ore no Imouto ga Konnani Kawaii Wake ga Nai. (saison 2) VOSTFR"
        );
        assert_eq!(
            remove_extension("Ep. 01 - The Boy in the Iceberg"),
            "Ep. 01 - The Boy in the Iceberg"
        );
        assert_eq!(
            remove_extension(
                "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"
            ),
            "Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"
        );
        assert_eq!(
            remove_extension("[LRL] 1001 Nights (1998) [DVD]"),
            "[LRL] 1001 Nights (1998) [DVD]"
        );
        assert_eq!(remove_extension("[Anime"), "[Anime");
        assert_eq!(
            remove_extension("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)"),
            "Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)"
        );
        assert_eq!(
            remove_extension("[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]"),
            "[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]"
        );
        assert_eq!(remove_extension("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]"),"[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]");
        assert_eq!(
            remove_extension("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)"),
            "[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)"
        );
        assert_eq!(
            remove_extension(
                "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
            ),
            "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
        );
        assert_eq!(
            remove_extension("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]"),
            "[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]"
        );
        assert_eq!(
            remove_extension(
                "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
            ),
            "[Doki] Nogizaka Haruka no Himitsu - Purezza - 01v2-03v2 (1280x720 h264 AAC)"
        );
        assert_eq!(
            remove_extension(
                "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
            ),
            "Fairy Tail - S06E32 - Tartaros Arc Iron Fist of the Fire Dragon [Episode 83]"
        );
        assert_eq!(
            remove_extension("Noragami - S02E06 - What Must Be Done [Episode 6]"),
            "Noragami - S02E06 - What Must Be Done [Episode 6]"
        );
        assert_eq!(
            remove_extension("[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]"),
            "[Harunatsu] Classroom Crisis - Vol.1 [BD 720p-AAC]"
        );
        assert_eq!(
            remove_extension("[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)"),
            "[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)"
        );
        assert_eq!(
            remove_extension("[Infantjedi] Norn9 - Norn + Nonetto - 12"),
            "[Infantjedi] Norn9 - Norn + Nonetto - 12"
        );
        assert_eq!(
            remove_extension("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA"),
            "Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA"
        );
        assert_eq!(
            remove_extension("[HorribleSubs] Momokuri - 01+02 [720p]"),
            "[HorribleSubs] Momokuri - 01+02 [720p]"
        );
        assert_eq!(remove_extension(""), "");
    }

    #[test]
    fn other_extension() {
        assert_eq!(
            remove_extension(
                "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch.7z"
            ),
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].patch"
        );
        assert_eq!(
            remove_extension("[Hard-Boiled FS]FullMetalAlchemist_09.rmvb"),
            "[Hard-Boiled FS]FullMetalAlchemist_09"
        );
    }

    #[test]
    fn try_bypass() {
        assert_eq!(
            remove_extension(
                "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch.7z"
            ),
            "[Yuurisan-Subs]_Darker_than_Black_-_Gemini_of_the_Meteor_-_01v2_[65274FDE].7z.patch"
        );
    }
}
