use anitomy_rust::parsing::extensions::get_extension;

#[test]
fn get_extension_mkv() {
    assert_eq!(get_extension("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").unwrap(),"mkv");
    assert_eq!(
        get_extension("[ANBU]_Princess_Lover!_-_01_[2048A39A].mkv").unwrap(),
        "mkv"
    );
    assert_eq!(
        get_extension("[ANBU-Menclave]_Canaan_-_01_[1024x576_H.264_AAC][12F00E89].mkv").unwrap(),
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
        get_extension("[HorribleSubs] Tower of Druaga - Sword of Uruk - 04 [480p].mkv").unwrap(),
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
        get_extension("[kito].Nazca.episode.01.DVDRip.[x264.He-aac.{Jpn}+Sub{Fr}].mkv").unwrap(),
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
        get_extension("[UTW-TMD]_Summer_Wars_[BD][h264-720p][TrueHD5.1][9F311DAB].mkv").unwrap(),
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
        get_extension("Evangelion_1.11_You_Are_(Not)_Alone_[1080p,BluRay,x264,DTS-ES]_-_THORA.mkv")
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
        get_extension("[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv")
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
        get_extension("[FuktLogik][Sayonara_Zetsubou_Sensei][01][DVDRip][x264_AC3].mkv").unwrap(),
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
        get_extension("[Jumonji-Giri]_[Shinsen-Subs][ASF]_D.C.II_Da_Capo_II_Ep01_(a1fc58a7).mkv")
            .unwrap(),
        "mkv"
    );
    assert_eq!(
        get_extension("[52wy][SlamDunk][001][Jpn_Chs_Cht][x264_aac][DVDRip][7FE2C873].mkv")
            .unwrap(),
        "mkv"
    );
    assert_eq!(
        get_extension("[Commie] Last Exile ~Fam, The Silver Wing~ - 13 [AFF9E530].mkv").unwrap(),
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
        get_extension("Evangelion Shin Gekijouban Q (BDrip 1920x1080 x264 FLACx2 5.1ch)-ank.mkv")
            .unwrap(),
        "mkv"
    );
    assert_eq!(
        get_extension("Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv")
            .unwrap(),
        "mkv"
    );
    assert_eq!(
        get_extension("[FFF] Futsuu no Joshikousei ga [Locodol] Yatte Mita. - 01 [BAD09C76].mkv")
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
        get_extension("[Urusai]_Bokura_Ga_Ita_01_[DVD_h264_AC3]_[BFCE1627][Fixed].mkv").unwrap(),
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
        get_extension("EvoBot.[Watakushi]_Akuma_no_Riddle_-_01v2_[720p][69A307A2].mkv").unwrap(),
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
        get_extension("The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv").unwrap(),
        "mkv"
    );
    assert_eq!(get_extension("[DmonHiro] Oreshura #01v2 - The Start Of High School Life Is A War Zone [BD, 720p] [211375E6].mkv").unwrap(),"mkv");
    assert_eq!(get_extension("[NinjaPanda] Tiger & Bunny #01 All's well that ends well. (v3, 1080p Hi10P, DA AAC) [4A9AB85F].mkv").unwrap(),"mkv");
    assert_eq!(
        get_extension("Neko no Ongaeshi - [HQR.remux-DualAudio][NTV.1280x692.h264](0CDC2145).mkv")
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
        get_extension("[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv")
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
        get_extension("[Ayako]_Infinite_Stratos_-_IS_-_01v2_[XVID][400p][29675B71].avi").unwrap(),
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
        get_extension("[[Zero-Raws] Shingeki no Kyojin - 05 (MBS 1280x720 x264 AAC).mp4").unwrap(),
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
        get_extension("[Hakugetsu&MGRT][Evangelion 3.0 You Can (Not) Redo][480P][V0].mp4").unwrap(),
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
        get_extension("Evangelion The New Movie Q (BD 1280x720 AVC AACx2 [5.1+2.0]).mp4").unwrap(),
        "mp4"
    );
    assert_eq!(
        get_extension("Kotonoha no Niwa (BD 1280x720 AVC AACx3 [5.1+2.0+2.0] Subx3).mp4").unwrap(),
        "mp4"
    );
    assert_eq!(
        get_extension("Queen's Blade Utsukushiki Toushi-tachi - OVA_01 (BD 1280x720 AVC AAC).mp4")
            .unwrap(),
        "mp4"
    );
    assert_eq!(
        get_extension("[異域字幕組][漆黑的子彈][Black Bullet][11][1280x720][繁体].mp4").unwrap(),
        "mp4"
    );
    assert_eq!(get_extension("[AoJiaoZero][Mangaka-san to Assistant-san to the Animation] 02 [BIG][X264_AAC][720P].mp4").unwrap(),"mp4");
    assert_eq!(
        get_extension("[모에-Raws] Abarenbou Rikishi!! Matsutarou #04 (ABC 1280x720 x264 AAC).mp4")
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
    assert!(get_extension("Hayate no Gotoku 2nd Season 24 (Blu-Ray 1080p) [Chihiro]").is_none());
    assert!(get_extension("[BluDragon] Blue Submarine No.6 (DVD, R2, Dual Audio) V3").is_none());
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
    assert!(
        get_extension("Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]")
            .is_none()
    );
    assert!(get_extension("[LRL] 1001 Nights (1998) [DVD]").is_none());
    assert!(get_extension("[Anime").is_none());
    assert!(get_extension("Gekkan Shoujo Nozaki-kun [HorribleSubs] (1080p)").is_none());
    assert!(
        get_extension("[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]")
            .is_none()
    );
    assert!(get_extension("[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]").is_none());
    assert!(
        get_extension("[DmonHiro] Magi - The Labyrinth Of Magic - Vol.1v2 (BD, 720p)").is_none()
    );
    assert!(get_extension(
        "[tlacatlc6] Natsume Yuujinchou Shi Vol. 1v2 & Vol. 2 (BD 1280x720 x264 AAC)"
    )
    .is_none());
    assert!(
        get_extension("[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]").is_none()
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
    assert!(get_extension("Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA").is_none());
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
