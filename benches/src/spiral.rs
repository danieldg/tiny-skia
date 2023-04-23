use test::Bencher;

const CURVES: &[f32] = &[
    497.55677,  502.45092,  497.54656,  502.45636,  497.54465,  502.45674,
    497.52276,  502.46118,  497.51289,  502.43207,  497.51519,  502.41554,
    497.52211,  502.36573,  497.58296,  502.35024,  497.62426,  502.36318,
    497.71148,  502.39051,  497.73433,  502.49836,  497.70231,  502.57476,
    497.64593,  502.7093,   497.47537,  502.74097,  497.35359,  502.68129,
    497.16165,  502.58723,  497.11983,  502.33818,  497.21579,  502.16079,
    497.35616,  501.90132,  497.69949,  501.84803,  497.9427,   501.98894,
    498.27987,  502.18426,  498.34591,  502.63764,  498.15141,  502.95689,
    497.89246,  503.38193,  497.31327,  503.462,    496.90777,  503.20524,
    496.38468,  502.87402,  496.2893,   502.15324,  496.61699,  501.65129,
    497.02915,  501.01995,  497.90729,  500.908,    498.5159,   501.31529,
    499.26566,  501.81705,  499.39547,  502.86832,  498.8999,   503.59378,
    498.29987,  504.47216,  497.0597,   504.62109,  496.2072,   504.02858,
    495.19001,  503.32161,  495.02068,  501.87678,  495.7188,   500.88703,
    496.54138,  499.72083,  498.20665,  499.52983,  499.34384,  500.34225,
    500.66923,  501.28911,  500.88317,  503.19058,  499.94779,  504.48541,
    498.86798,  505.98018,  496.71454,  506.21833,  495.25188,  505.15132,
    493.57753,  503.92989,  493.31389,  501.50871,  494.52121,  499.86802,
    495.89294,  498.0039,   498.59763,  497.71351,  500.42654,  499.06981,
    502.49062,  500.6005,   502.80903,  503.60447,  501.29508,  505.6318,
    499.59675,  507.90603,  496.27774,  508.25374,  494.0418,   506.57346,
    491.54722,  504.69882,  491.16895,  501.049,    493.02422,  498.59425,
    495.08385,  495.86913,  499.08025,  495.45902,  501.764,    497.49797,
    504.72985,  499.75125,  505.17306,  504.11,     502.94177,  507.03294,
    500.48616,  510.24972,  495.74929,  510.72731,  492.57697,  508.295,
    489.09907,  505.62839,  488.58584,  500.49764,  491.22784,  497.06573,
    494.11412,  493.31652,  499.65452,  492.76637,  503.3562,   495.62674,
    507.38692,  498.74136,  507.97526,  504.70719,  504.88785,  508.68883,
    501.53621,  513.01125,  495.12919,  513.63905,  490.85739,  510.31593,
    486.23308,  506.71859,  485.56455,  499.85462,  489.13206,  495.28246,
    492.98376,  490.34607,  500.32045,  489.63554,  505.20316,  493.4561,
    510.46183,  497.57084,  511.21563,  505.39603,  507.13334,  510.59948,
    502.74689,  516.19062,  494.41744,  516.98896,  488.88305,  512.63626,
    482.94924,  507.96943,  482.1051,   499.11996,  486.73687,  493.24444,
    491.69276,  486.95777,  501.07803,  486.06655,  507.30487,  490.98607,
    513.95459,  496.23969,  514.89416,  506.17652,  509.67821,  512.76488,
    504.11819,  519.78784,  493.61403,  520.77703,  486.65397,  515.25599,
    479.24757,  509.38089,  478.20748,  498.29363,  484.0423,   490.95167,
    490.24114,  483.15164,  501.92727,  482.05939,  509.66133,  488.21665,
    517.86519,  494.74791,  519.01087,  507.04868,  512.52249,  515.18503,
    505.65013,  523.8029,   492.71896,  525.00328,  484.17013,  518.17512,
    475.12805,  510.95299,  473.87169,  497.37565,  481.04832,  488.40414,
    488.62889,  478.92766,  502.86817,  477.61406,  512.27255,  485.14782,
    522.19363,  493.09551,  523.56574,  508.01249,  515.66617,  517.85994,
    507.34269,  528.2358,   491.73223,  529.6677,   481.43153,  521.39364,
    470.59068,  512.68571,  469.09773,  496.366,    477.75495,  485.60186,
    486.85602,  474.28583,  503.90073,  472.73055,  515.13852,  481.7796,
    526.93991,  491.28247,  528.55879,  509.06797,  519.10924,  520.78959,
    509.19588,  533.08655,  490.65384,  534.77029,  478.43819,  524.91156,
    465.63548,  514.57905,  463.8856,   495.2647,   474.16218,  482.54482,
    484.92251,  469.22616,  505.02495,  467.40888,  518.25924,  478.11198,
    532.10404,  489.30881,  533.99,     510.2151,   522.85171,  523.974,
    511.2097,   538.35514,  489.48379,  540.31104,  475.19009,  528.72888,
    460.26242,  516.63303,  458.2353,   494.07173,  470.27001,  479.23303,
    482.82838,  463.74865,  506.24083,  461.64904,  521.63472,  474.14496,
    537.68602,  487.17453,  539.85938,  511.4539,   526.89358,  527.41317,
    513.38414,  544.04157,  488.22208,  546.28997,  471.68724,  532.8456,
    454.47153,  518.84763,  452.14684,  492.78711,  466.07844,  475.66649,
    480.57362,  457.85329,  507.54837,  455.45103,  525.26495,  469.87854,
    543.68584,  484.87961,  546.16694,  512.78435,  531.23484,  531.10708,
    515.71921,  550.14585,  486.86871,  552.70707,  467.92963,  537.26171,
    448.26279,  521.22285,  445.6202,   491.41082,  461.58748,  471.8452,
    478.15824,  451.54009,  508.94757,  448.81485,  529.14993,  465.31273,
    550.1035,   482.42407,  552.91266,  514.20647,  535.8755,   535.05575,
    518.21491,  556.66798,  485.42368,  559.56233,  463.91727,  541.97723,
    441.6362,   523.75871,  438.65539,  489.94288,  456.79712,  467.76915,
    475.58223,  444.80904,  510.43843,  441.7405,   533.28966,  460.44752,
    556.939,    479.80791,  560.09655,  515.72024,  540.81556,  539.25918,
    520.87123,  563.60795,  483.88699,  566.85577,  459.65016,  546.99214,
    434.59177,  526.45518,  431.25241,  488.38327,  451.70736,  463.43836,
    472.8456,   437.66015,  512.02095,  434.22798,  537.68415,  455.28291,
    564.19236,  477.03112,  567.71862,  517.32568,  546.05502,  543.71735,
    523.68818,  570.96576,  482.25864,  574.58737,  455.1283,   552.30644,
    427.1295,   529.31229,  423.41126,  486.73201,  446.3182,   458.8528,
    469.94834,  430.09342,  513.69513,  426.27729,  542.33339,  449.8189,
    571.86355,  474.0937,   575.77885,  519.02278,  551.59388,  548.43028,
    526.66575,  578.74141,  480.53863,  582.75715,  450.35168,  557.92015,
    419.24938,  532.33002,  415.13195,  484.98908,  440.62965,  454.0125,
    466.89045,  422.10884,  515.46098,  417.88843,  547.23738,  444.0555,
    579.95259,  470.99566,  584.27725,  520.81154,  557.43213,  553.39796,
    529.80395,  586.93492,  478.72695,  591.36509,  445.32032,  563.83325,
    410.95142,  535.50837,  406.41446,  483.15449,  434.64169,  448.91744,
    463.67194,  413.70642,  517.31848,  409.0614,   552.39613,  437.9927,
    588.45948,  467.73699,  593.21382,  522.69196,  563.56978,  558.62039,
    533.10277,  595.54626,  476.82362,  600.4112,   440.03419,  570.04575,
    402.23561,  538.84735,  397.2588,   481.22824,  428.35434,  443.56763,
    460.2928,   404.88615,  519.26765,  399.7962,   557.80962,  431.6305,
    597.3842,   464.3177,   602.58856,  524.66404,  570.00683,  564.09758,
    536.56223,  604.57545,  474.82862,  609.89549,  434.49332,  576.55765,
    393.10196,  542.34696,  387.66498,  479.21033,  421.7676,   437.96307,
    456.75304,  395.64804,  521.30847,  390.09283,  563.47788,  424.9689,
    606.72678,  460.73778,  612.40148,  526.72778,  576.74328,  569.82952,
    540.1823,   614.02248,  472.74196,  619.81794,  428.69769,  583.36894,
    383.55047,  546.00719,  377.63298,  477.10075,  414.88145,  432.10375,
    453.05265,  385.99208,  523.44096,  379.95129,  569.40088,  418.00791,
    616.4872,   456.99723,  622.65256,  528.88318,  583.77912,  575.81621,
    543.963,    623.88736,  470.56365,  630.17857,  422.64731,  590.47964,
    373.58113,  549.82805,  367.16281,  474.89952,  407.69591,  425.98969,
    449.19164,  375.91828,  525.66511,  369.37158,  575.57864,  410.74752,
    626.66546,  453.09606,  633.34181,  531.13024,  591.11436,  582.05766,
    547.90433,  634.17009,  468.29367,  640.97736,  416.34218,  597.88973,
    363.19395,  553.80954,  356.25448,  472.60663,  400.21097,  419.62086,
    445.17,     365.42664,  527.98092,  358.35371,  582.01115,  403.18773,
    637.26156,  449.03426,  644.46923,  533.46897,  598.749,    588.55385,
    552.00628,  644.87065,  465.93203,  652.21432,  409.78229,  605.59921,
    352.38892,  557.95164,  344.90797,  470.22207,  392.42663,  412.99729,
    440.98773,  354.51715,  530.38839,  346.89766,  588.69841,  395.32854,
    648.27551,  444.81184,  656.03482,  535.89936,  606.68303,  595.3048,
    556.26886,  655.98907,  463.47873,  663.88946,  402.96765,  613.6081,
    341.16604,  562.25438,  333.1233,   467.74585,  384.3429,   406.11896,
    436.64484,  343.18981,  532.88752,  335.00344,  595.64042,  387.16996,
    659.70731,  440.42879,  668.03858,  538.4214,   614.91647,  602.31051,
    560.69207,  667.52532,  460.93376,  676.00276,  395.89826,  621.91638,
    329.52533,  566.71774,  320.90045,  465.17798,  375.95977,  398.98588,
    432.14132,  331.44463,  535.47832,  322.67105,  602.83719,  378.71198,
    671.55695,  435.88512,  680.48051,  541.03511,  623.4493,   609.57096,
    565.2759,   679.47942,  458.29714,  688.55423,  388.57412,  630.52406,
    317.46676,  571.34173,  308.23944,  462.51844,  367.27723,  391.59805,
    427.47718,  319.28161,  538.16077,  309.9005,   610.28871,  369.9546,
    683.82443,  431.18082,  693.36061,  543.74048,  632.28153,  617.08617,
    570.02035,  691.85137,  455.56885,  701.54387,  380.99522,  639.43114,
    304.99036,  576.12634,  295.14025,  459.76724,  358.29531,  383.95546,
    422.65241,  306.70074,  540.93489,  296.69177,  617.99499,  360.89782,
    696.50976,  426.3159,   706.67888,  546.53751,  641.41316,  624.85614,
    574.92543,  704.64116,  452.74891,  714.97168,  373.16157,  648.63762,
    292.09611,  581.07157,  281.6029,   456.92437,  349.01398,  376.05813,
    417.66702,  293.70203,  543.80066,  283.04487,  625.95601,  351.54164,
    709.61293,  421.29035,  720.43532,  549.42621,  650.84418,  632.88085,
    579.99114,  717.84879,  449.8373,   728.83767,  365.07317,  658.14349,
    278.78401,  586.17744,  267.62738,  453.98985,  339.43326,  367.90603,
    412.521,    280.28548,  546.7581,   268.95981,  634.17179,  341.88607,
    723.13395,  416.10417,  734.62993,  552.40656,  660.5746,   641.16032,
    585.21747,  731.47427,  446.83403,  743.14182,  356.73001,  667.94876,
    265.05407,  591.44393,  253.21368,  450.96367,  329.55314,  359.49919,
    407.21436,  266.45108,  549.8072,   254.43657,  642.64233,  331.9311,
    737.07281,  410.75737,  749.2627,   555.47857,  670.60442,  649.69454,
    590.60443,  745.51759,  443.7391,   757.88414,  348.1321,   678.05343,
    250.90629,  596.87104,  238.36182,  447.84582,  319.37362,  350.83759,
    401.74709,  252.19883,  552.94796,  239.47516,  651.36761,  321.67674,
    751.42952,  405.24995,  764.33365,  558.64225,  680.93364,  658.48351,
    596.15201,  759.97876,  440.55251,  773.06463,  339.27944,  688.4575,
    236.34066,  602.45878,  223.07178,  444.63632,  308.8947,   341.92124,
    396.11919,  237.52874,  556.18038,  224.07559,  660.34765,  311.12297,
    766.20407,  399.58189,  779.84277,  561.89759,  691.56225,  667.52724,
    601.86022,  774.85777,  437.27426,  788.68329,  330.17202,  699.16096,
    221.35719,  608.20714,  207.34358,  441.33515,  298.11639,  332.75014,
    390.33067,  222.44081,  559.50447,  208.23784,  669.58244,  300.26981,
    781.39646,  393.75322,  795.79006,  565.24458,  702.49026,  676.82572,
    607.72905,  790.15463,  433.90434,  804.74012,  320.80986,  710.16382,
    205.95587,  614.11613,  191.17721,  437.94232,  287.03868,  323.32428,
    384.38152,  206.93503,  562.92021,  191.96193,  679.07198,  289.11725,
    797.0067,   387.76391,  812.17552,  568.68324,  713.71767,  686.37895,
    613.75851,  805.86933,  430.44277,  821.23512,  311.19294,  721.46608,
    190.13671,  620.18575,  174.57267,  434.45783,  275.66157,  313.64368,
    378.27175,  191.01141,  566.42762,  175.24784,  688.81628,  277.66529,
    813.03479,  381.61399,  828.99914,  572.21356,  725.24448,  696.18694,
    619.94859,  822.00187,  426.88953,  838.16829,  301.32126,  733.06774,
    173.8997,   626.41599,  157.52995,  430.88168,  263.98507,  303.70831,
    372.00135,  174.66994,  570.02668,  158.09559,  698.81533,  265.91393,
    829.48072,  375.30343,  846.26094,  575.83555,  737.07069,  706.24967,
    626.2993,   838.55226,  423.24464,  855.53963,  291.19484,  744.96879,
    157.24485,  632.80685,  140.04907,  427.21386,  252.00916,  293.5182,
    365.57033,  157.91063,  573.71741,  140.50516,  709.06913,  253.86318,
    846.34449,  368.83225,  863.96091,  579.54919,  749.19629,  716.56716,
    632.81064,  855.5205,   419.50808,  873.34914,  280.81366,  757.16925,
    140.17216,  639.35835,  122.13002,  423.45439,  239.73386,  283.07333,
    358.97868,  140.73347,  577.4998,   122.47657,  719.57769,  241.51303,
    863.62611,  362.20045,  882.09904,  583.35449,  761.62129,  727.13941,
    639.4826,   872.90657,  415.67986,  891.59682,  270.17773,  769.6691,
    122.68162,  646.07046,  103.7728,   419.60326,  227.15916,  272.37371,
    352.22641,  123.13847,  581.37385,  104.00981,  730.341,    228.86348,
    881.32557,  355.40802,  900.67535,  587.25146,  774.34569,  737.9664,
    646.31519,   890.7105,  411.75998,  910.28266,  259.28704,  782.46834,
    104.77323,  652.94321,  84.977409,  415.66046,  214.28507,  261.41934,
    345.31351,  105.12563,  585.33956,  85.104873,  741.35906,  215.91454,
    899.44287,  348.45496,  919.68982,  591.24009,  787.36948,  749.04815,
    653.3084,   908.93226,  407.74843,  929.40668,  248.1416,   795.56699,
    86.447006,  659.97658,  65.743848,  411.626,    201.11157,  250.21021,
    338.23999,  86.694936,  589.39693,  65.761769,  752.63187,  202.66619,
    917.97802,  341.34128,  939.14247,  595.32038,  800.69267,  760.38466,
    660.46224,  927.57188,  403.64523,  948.96887,  236.74141,  808.96503,
    67.702934,  667.17057,  46.072117,  407.49988,  187.63868,  238.74634,
    331.00583,  67.846403,  593.54597,  45.980496,  764.15944,  189.11845,
    936.93102,  334.06698,  959.03329,  599.49232,  814.31527,  771.97591,
    667.7767,   946.62933,  399.45037,  968.96923,  225.08647,  822.66247,
    48.541016,  674.52519,  25.962217,  403.2821,   173.86639,  227.0277,
    323.61106,  48.580024,  597.78666,  25.761053,  775.94176,  175.27131,
    956.30186,  326.63204,  979.36227,  603.75594,  828.23725,  783.82192,
    675.25179,  966.10463,  395.16384,  989.40776,  213.17677,  836.65931,
    28.961255,  682.04043,  5.4141471,  398.97266,   159.7947,  215.05432,
    316.05566,  28.895802,  602.11902,  5.1034409,  787.97883,  161.12478,
    976.09054,  319.03649,  1000.1294,  608.11121,  842.45864,  795.92268,
     682.8875,  985.99778,  390.78565,  1010.2845,  201.01233,  850.95554,
    8.9636489,   689.7163, -15.572093,  394.57156,  145.42362,  202.82618,
    308.33963,  8.7937345,  606.54304, -15.992341,  800.27066,  146.67884,
    996.29707,   311.2803,  1021.3348,  612.55814,  856.97942,  808.27819,
    690.68384,  1006.3088,   386.3158,  1031.5993,  188.59312,  865.55117,
   -11.451802,   697.5528, -36.996502,   390.0788,  130.75314,  190.34329,
    300.46298, -11.726177,  611.05872, -37.526293,  812.81723,  131.93351,
    1016.9214,  303.36349,  1042.9782,  617.09673,   871.7996,  820.88846,
];

#[bench]
fn tiny_skia(bencher: &mut Bencher) {
    use tiny_skia::*;

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = false;

    let mut pb = PathBuilder::new();
    pb.move_to(497.55261, 502.44739);
    for c in CURVES.chunks(6) {
        pb.cubic_to(c[0], c[1], c[2], c[3], c[4], c[5]);
    }
    let path = pb.finish().unwrap();

    let mut pixmap = Pixmap::new(1000, 1000).unwrap();
    bencher.iter(|| {
        pixmap.stroke_path(&path, &paint, &Stroke::default(), Transform::identity(), None);
    });
}

#[cfg(feature = "skia-rs")]
#[bench]
fn skia(bencher: &mut Bencher) {
    use skia_rs::*;

    let mut surface = Surface::new_rgba_premultiplied(1000, 1000).unwrap();

    let mut paint = Paint::new();
    paint.set_color(50, 127, 150, 200);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(1.0);
    paint.set_blend_mode(BlendMode::SourceOver);
    paint.set_anti_alias(false);

    let mut path = Path::new();
    path.move_to(497.55261, 502.44739);
    for c in CURVES.chunks(6) {
        path.cubic_to(c[0], c[1], c[2], c[3], c[4], c[5]);
    }

    bencher.iter(|| {
        surface.draw_path(&path, &paint);
    });
}

#[cfg(feature = "raqote")]
#[bench]
fn raqote(bencher: &mut Bencher) {
    use raqote::*;

    let mut dt = DrawTarget::new(1000, 1000);

    let mut path = {
        let mut pb = PathBuilder::new();
        pb.move_to(497.55261, 502.44739);
        for c in CURVES.chunks(6) {
            pb.cubic_to(c[0], c[1], c[2], c[3], c[4], c[5]);
        }
        pb.finish()
    };
    path.winding = Winding::EvenOdd;

    // raqote uses ARGB order.
    let src = Source::from(Color::new(200, 50, 127, 150));

    let draw_opt = DrawOptions {
        blend_mode: BlendMode::SrcOver,
        alpha: 1.0,
        antialias: AntialiasMode::None,
    };

    let style = StrokeStyle::default();

    bencher.iter(|| {
        dt.stroke(&path, &src, &style, &draw_opt);
    });
}

#[cfg(feature = "cairo-rs")]
#[bench]
fn cairo(bencher: &mut Bencher) {
    use cairo::*;

    let surface = ImageSurface::create(Format::ARgb32, 1000, 1000).unwrap();

    let cr = Context::new(&surface);

    cr.move_to(497.55261, 502.44739);
    for c in CURVES.chunks(6) {
        cr.curve_to(c[0] as f64, c[1] as f64, c[2] as f64, c[3] as f64, c[4] as f64, c[5] as f64);
    }

    cr.set_source_rgba(50.0 / 255.0, 127.0 / 255.0, 150.0 / 255.0, 200.0 / 255.0);
    cr.set_antialias(Antialias::None);
    cr.set_operator(Operator::Over);

    bencher.iter(|| {
        // Does it cache the stoked path?
        cr.stroke_preserve();
    });
}
