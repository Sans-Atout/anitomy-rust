use anitomy_rust::parsing::extensions::remove_extension;

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
        remove_extension("[Chihiro]_Kono_Aozora_ni_Yakusoku_Wo_10_v2_[DVD][h264][C83D206B].mkv"),
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
        remove_extension("Magical Girl Lyrical Nanoha A's - 01.DVD[H264.AAC][DGz][7A8A7769].mkv"),
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
        remove_extension("[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A].mkv"),
        "[Seto_Otaku]_AIKa_ZERO_OVA_-_01_[BD][1920x1080_H264-Flac][6730D40A]"
    );
    assert_eq!(
        remove_extension("[a4e]R.O.D_the_TV_01[divx5.2.1].mkv"),
        "[a4e]R.O.D_the_TV_01[divx5.2.1]"
    );
    assert_eq!(remove_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"),"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep05v2_EXCAVATION_[720p,HDTV,x264,AAC_5.1]_-_THORA");
    assert_eq!(remove_extension("Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA.mkv"),"Ghost_in_the_Shell_Stand_Alone_Complex_2nd_GIG_Ep06_Pu239_[720p,HDTV,x264,AAC_5.1]_-_THORA");
    assert_eq!(
        remove_extension("Fate_Stay_Night_Ep05_The_Two_Magi_Part1_[720p,BluRay,x264]_-_THORA.mkv"),
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
        remove_extension("[EroGaKi-Team]_Nurse_Witch_Komugi-chan_Magikarte_02.5_[902BB314].mkv"),
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
        remove_extension("Aim_For_The_Top!_Gunbuster-ep1.BD(H264.FLAC.10bit)[KAA][69ECCDCF].mkv"),
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
        remove_extension("Howl's_Moving_Castle_(2004)_[1080p,BluRay,flac,dts,x264]_-_THORA v2.mkv"),
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
        remove_extension("[Coalgirls]_Bakemonogatari_OP4a_(1280x720_Blu-Ray_FLAC)_[327A2375].mkv"),
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
        remove_extension("[Hatsuyuki]_Kuroko_no_Basuke_S3_-_01_(51)_[720p][10bit][619C57A0].mkv"),
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
        remove_extension("[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588].mkv"),
        "[FFF] Seirei Tsukai no Blade Dance - SP01 [BD][720p-AAC][F1FF8588]"
    );
    assert_eq!(
        remove_extension("[Hatsuyuki] Dragon Ball Kai (2014) - 002 (100) [1280x720][DD66AFB7].mkv"),
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
        remove_extension("[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv"),
        "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5]"
    );
    assert_eq!(
        remove_extension("[FBI] Baby Princess 3D Paradise Love 01v0 [BD][720p-AAC][457CC066].mkv"),
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
        remove_extension("[바카-Raws] Nekomonogatari (Black) #1-4 (BS11 1280x720 x264 AAC).mp4"),
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
        remove_extension("Byousoku 5 Centimeter [Blu-Ray][1920x1080 H.264][2.0ch AAC][SOFTSUBS]"),
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
