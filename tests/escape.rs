use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn to_json<T: serde::Serialize + ?Sized>(value: &T) -> String {
    evil_json::to_string(value).unwrap()
}

fn naive_escape(feed: &str) -> String {
    use std::io::Write;
    let mut ret = Vec::with_capacity(feed.len());
    ret.push(b'"');

    for byte in feed.bytes() {
        let slice = match byte {
            b'\t' => b"\\t",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            0x08 => b"\\b",
            0x0c => b"\\f",
            0x00..=0x1f => {
                write!(ret, "\\u{:04x}", byte).unwrap();
                continue;
            }
            _ => {
                ret.push(byte);
                continue;
            }
        };

        ret.extend(slice);
    }

    ret.push(b'"');
    String::from_utf8(ret).unwrap()
}

#[test]
fn no_escape() {
    assert_eq!(to_json(""), "\"\"");
    assert_eq!(to_json("apple"), "\"apple\"");
    assert_eq!(
        to_json("!#$%&'()-=^~|@`[{;+:*]},<.>/?_"),
        "\"!#$%&'()-=^~|@`[{;+:*]},<.>/?_\""
    );
    assert_eq!(
        to_json("漢字はエスケープしないはずだよ"),
        "\"漢字はエスケープしないはずだよ\""
    );
}

#[test]
fn short() {
    assert_eq!(to_json("\""), r#""\"""#);
    assert_eq!(to_json("\x08\x09\x0a\x0c\x0d\"\\"), r#""\b\t\n\f\r\"\\""#);
    assert_eq!(
        to_json(r#"{"title": "This is a JSON!"}"#),
        r#""{\"title\": \"This is a JSON!\"}""#
    );
}

#[test]
#[rustfmt::skip]
fn long() {
    assert_eq!(
        to_json("E8\u{10}^\u{6}m\nrL&o\u{14}\u{1b})e G\u{8}<\u{1e}Mr/]S\ti\u{13}\u{f}\\\u{b};B7\u{6}qTBv_\u{1e}?dZVi<\u{2}w7KYs\u{18}~LebV\u{8}!8\u{1a}$\u{6}vJ\u{1}hsAv\u{1c}t\'x#c\r\u{3}\u{2}\ne\t\'F\u{16}$49uSP\u{15}\u{1c}w\u{1f}]/$8G&e\u{f}Z1R,=H&>naAHn@1%\u{0}#ji2\u{1}\u{1}>/i\']T\u{1a}]E-|\u{14}!YW-i\"!qqSMF%qWG \u{11}3q\u{1b}R?qtq3\u{0}>*P0k\u{1}.}\u{c}nGlO_b\u{14}HwtnG\u{2}#B\u{1b}\u{11}\u{1e}}mKD\u{8}\u{7}MPk\u{4}kLS0Kd)qJc9\u{18}a\u{8}n+Hh \u{15}\u{1d}\u{12}v@\\\u{3}T6]>*\t\u{1d}\u{0}5Q5+Hch kT\u{1a}\u{1a}JD\u{19}%i\u{1e}8jb3J@2\u{1c}\u{14}u3\u{17}m\u{c}nko\u{6}m\nK)s\u{8}KC=`\nfKbRD\u{7}\u{1f}Dr2T\r&|OC\u{1e}\r|\u{1e}\u{c}\u{1e}\u{17}45U\u{10}JY0@\u{c}ZN]\u{f}/mG3\u{b}yUrC~sr{\u{17}v1\\\u{3}Y\u{1}\u{12}COVm a\u{11}Bi\u{f}X\u{5}+?C\t\r2Y\u{2}RW7\nHI^\u{1b}{r\u{18}\nD\u{3}y4RWS,\u{1e}g\u{1b}Z\u{6}~ 8g_=e\rUC3~9p)v\u{12}\u{18}jG\u{10}f!V\u{8}\u{17}tej1:\u{e}wAp5\u{19}m_])PVxL[I#\u{c}f3J)dA53\u{16}b:\u{15}\r_*0lA\u{1d}\u{19}i\u{f}v\u{1b}R-sU(\r\u{13}}D\u{13},XL]{@1r\nKo\u{16}| \u{1b}?:\u{1f}`/opDLn20T2M\u{1c}U\u{14}{\t(\u{15}Ri&p\u{c}Cq-]\u{1e}j\u{11}a\\\u{1e}8]C^iRPRLx]\\>\u{1b}Fz\u{17}xf~9\u{15}N\u{13}\u{e}]tudm\u{18}\u{7}Fs\u{10}\u{15}:{\u{19}r0G\u{1b}~if\u{1f}Ck\\%\u{e}VL=!H\u{7}Kz{nhp50oI\u{1e}LUdr\u{2}+\u{6}\u{1f}\u{b}\u{7}\u{15}o}g$eV8\u{f}\u{f}g\u{5}\u{1b}iv\u{16}lg+\'4gjXr1P\u{19}W\u{17}\u{19}k[g&A<\u{18}Ka`\u{16}LyO\u{e}?\r\u{1e}38?C\\l>\u{3}zZBScL\u{1a}\u{7}d\u{17}6\u{1f}U0\u{3}I7X1V\u{f},f]6\u{4}\u{13}\u{17}\u{19}\u{1c}Pjg*XT\u{7}Zp\u{14}=\u{15}\u{15}#3|\u{17}\u{7}z\u{11}\"|XJxj\u{19}/\u{5}\u{11}y\u{10}NVf2{LOZ_q(\u{0}:Ip>\u{1c}^S)wB\'tXISN\u{16}b=>h\u{15}WC{pZ\u{13}B/n*\u{3}kg99\u{7}\u{6}sp[\u{1c}\u{7}3\u{b}Ukd]\'?0\u{0}o\"\u{5}~Szm5>\u{1d}g(FT~!HVn\u{7}J.\u{f}k<5\u{12}\u{e}?w\u{4}N`k\u{f}>;\\=ka\u{13}p\u{10}^;K+`D59\u{4}zihV1!\u{1e}\u{6}\u{1b}EID\u{1d}G[VP\u{3}-I!g(3\u{3}M\u{5}\r=xqq\u{1a}\u{1c}0\u{5}\u{1c}\u{e}U;[4\u{18};V\u{1}\u{16}\u{0}\u{7}7\u{1b}\"B%%H/s{NF_##esR\u{1d}\u{3}>\u{1b}d\u{1b}\u{13} gC\u{3}\u{5}ws6I=\u{1f}Ql%P^5Y6~\'F\u{19}pF-v\u{1d} \"[n6|olER\u{e}C\u{13}\u{5}B\u{0}Q4\u{1d}1f2@c\u{1d}\u{16}\u{c}g\u{8}W\u{c}\u{10}\u{5}<6<\"B\\fJ~;\u{8}.)`\u{10}Y\u{7}*#\u{10}\\\u{16}CRI"),
        "\"E8\\u0010^\\u0006m\\nrL&o\\u0014\\u001b)e G\\b<\\u001eMr/]S\\ti\\u0013\\u000f\\\\\\u000b;B7\\u0006qTBv_\\u001e?dZVi<\\u0002w7KYs\\u0018~LebV\\b!8\\u001a$\\u0006vJ\\u0001hsAv\\u001ct\'x#c\\r\\u0003\\u0002\\ne\\t\'F\\u0016$49uSP\\u0015\\u001cw\\u001f]/$8G&e\\u000fZ1R,=H&>naAHn@1%\\u0000#ji2\\u0001\\u0001>/i\']T\\u001a]E-|\\u0014!YW-i\\\"!qqSMF%qWG \\u00113q\\u001bR?qtq3\\u0000>*P0k\\u0001.}\\fnGlO_b\\u0014HwtnG\\u0002#B\\u001b\\u0011\\u001e}mKD\\b\\u0007MPk\\u0004kLS0Kd)qJc9\\u0018a\\bn+Hh \\u0015\\u001d\\u0012v@\\\\\\u0003T6]>*\\t\\u001d\\u00005Q5+Hch kT\\u001a\\u001aJD\\u0019%i\\u001e8jb3J@2\\u001c\\u0014u3\\u0017m\\fnko\\u0006m\\nK)s\\bKC=`\\nfKbRD\\u0007\\u001fDr2T\\r&|OC\\u001e\\r|\\u001e\\f\\u001e\\u001745U\\u0010JY0@\\fZN]\\u000f/mG3\\u000byUrC~sr{\\u0017v1\\\\\\u0003Y\\u0001\\u0012COVm a\\u0011Bi\\u000fX\\u0005+?C\\t\\r2Y\\u0002RW7\\nHI^\\u001b{r\\u0018\\nD\\u0003y4RWS,\\u001eg\\u001bZ\\u0006~ 8g_=e\\rUC3~9p)v\\u0012\\u0018jG\\u0010f!V\\b\\u0017tej1:\\u000ewAp5\\u0019m_])PVxL[I#\\ff3J)dA53\\u0016b:\\u0015\\r_*0lA\\u001d\\u0019i\\u000fv\\u001bR-sU(\\r\\u0013}D\\u0013,XL]{@1r\\nKo\\u0016| \\u001b?:\\u001f`/opDLn20T2M\\u001cU\\u0014{\\t(\\u0015Ri&p\\fCq-]\\u001ej\\u0011a\\\\\\u001e8]C^iRPRLx]\\\\>\\u001bFz\\u0017xf~9\\u0015N\\u0013\\u000e]tudm\\u0018\\u0007Fs\\u0010\\u0015:{\\u0019r0G\\u001b~if\\u001fCk\\\\%\\u000eVL=!H\\u0007Kz{nhp50oI\\u001eLUdr\\u0002+\\u0006\\u001f\\u000b\\u0007\\u0015o}g$eV8\\u000f\\u000fg\\u0005\\u001biv\\u0016lg+\'4gjXr1P\\u0019W\\u0017\\u0019k[g&A<\\u0018Ka`\\u0016LyO\\u000e?\\r\\u001e38?C\\\\l>\\u0003zZBScL\\u001a\\u0007d\\u00176\\u001fU0\\u0003I7X1V\\u000f,f]6\\u0004\\u0013\\u0017\\u0019\\u001cPjg*XT\\u0007Zp\\u0014=\\u0015\\u0015#3|\\u0017\\u0007z\\u0011\\\"|XJxj\\u0019/\\u0005\\u0011y\\u0010NVf2{LOZ_q(\\u0000:Ip>\\u001c^S)wB\'tXISN\\u0016b=>h\\u0015WC{pZ\\u0013B/n*\\u0003kg99\\u0007\\u0006sp[\\u001c\\u00073\\u000bUkd]\'?0\\u0000o\\\"\\u0005~Szm5>\\u001dg(FT~!HVn\\u0007J.\\u000fk<5\\u0012\\u000e?w\\u0004N`k\\u000f>;\\\\=ka\\u0013p\\u0010^;K+`D59\\u0004zihV1!\\u001e\\u0006\\u001bEID\\u001dG[VP\\u0003-I!g(3\\u0003M\\u0005\\r=xqq\\u001a\\u001c0\\u0005\\u001c\\u000eU;[4\\u0018;V\\u0001\\u0016\\u0000\\u00077\\u001b\\\"B%%H/s{NF_##esR\\u001d\\u0003>\\u001bd\\u001b\\u0013 gC\\u0003\\u0005ws6I=\\u001fQl%P^5Y6~\'F\\u0019pF-v\\u001d \\\"[n6|olER\\u000eC\\u0013\\u0005B\\u0000Q4\\u001d1f2@c\\u001d\\u0016\\fg\\bW\\f\\u0010\\u0005<6<\\\"B\\\\fJ~;\\b.)`\\u0010Y\\u0007*#\\u0010\\\\\\u0016CRI\""
    );
}

#[test]
#[cfg(not(miri))]
fn random() {
    let mut rng = SmallRng::seed_from_u64(0x9c13a55cd027849a);

    for len in 1..=100 {
        for _ in 0..100 {
            let bytes: Vec<u8> = (0..len).map(|_| rng.gen_range(0..0x80)).collect();
            let feed = std::str::from_utf8(bytes.as_slice()).unwrap();

            assert_eq!(to_json(feed), naive_escape(feed));
        }
    }
}
