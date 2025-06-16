use super::{LintGroup, MapPhraseLinter};

/// Produce a [`LintGroup`] that looks for errors in common phrases.
/// Comes pre-configured with the recommended default settings.
pub fn lint_group() -> LintGroup {
    let mut group = LintGroup::default();

    macro_rules! add_exact_mappings {
        ($group:expr, {
            $($name:expr => ($input:expr, $corrections:expr, $hint:expr, $description:expr)),+ $(,)?
        }) => {
            $(
                $group.add_pattern_linter(
                    $name,
                    Box::new(
                        MapPhraseLinter::new_exact_phrases(
                            $input,
                            $corrections,
                            $hint,
                            $description
                        ),
                    ),
                );
            )+
        };
    }

    add_exact_mappings!(group, {
        "AfterAWhile" => (
            ["after while"],
            ["after a while"],
            "When describafterg a timeframe, use `a while`.",
            "Corrects the missing article after `after while` or `after awhile`, forming `after a while`."
        ),
        "ALongTime" => (
            ["along time"],
            ["a long time"],
            "Use `a long time` for referring to a duration of time.",
            "Corrects `along time` to `a long time`."
        ),
        "ALotWorst" => (
            ["a lot worst", "alot worst"],
            ["a lot worse"],
            "Use `worse` for comparing. (`Worst` is for the extreme case)",
            "Corrects `a lot worst` to `a lot worse` for proper comparative usage."
        ),
        "AlzheimersDisease" => (
            ["old-timers' disease"],
            ["Alzheimer’s disease"],
            "Use the correct medical term.",
            "Fixes the common misnomer `old-timers' disease`, ensuring the correct medical term `Alzheimer’s disease` is used."
        ),
        "AnAnother" => (
            ["an another", "a another"],
            ["another"],
            "Use `another` on its own.",
            "Corrects `an another` and `a another`."
        ),
        "AndIn" => (
            ["an in"],
            ["and in"],
            "Did you mean `and in`?",
            "Fixes the incorrect phrase `an in` to `and in` for proper conjunction usage."
        ),
        "AndTheLike" => (
            ["an the like"],
            ["and the like"],
            "Did you mean `and the like`?",
            "Fixes the typo in `and the like`."
        ),
        "AnotherAn" => (
            ["another an"],
            ["another"],
            "Use `another` on its own.",
            "Corrects `another an` to `another`."
        ),
        "AnotherOnes" => (
            ["another ones"],
            ["another one", "another one's", "other ones"],
            "`another` is singular but `ones` is plural. Or maybe you meant the possessive `one's`.",
            "Corrects `another ones`."
        ),
        "AnotherThings" => (
            ["another things"],
            ["another thing", "other things"],
            "`another` is singular but `things` is plural.",
            "Corrects `another things`."
        ),
        "AsFarBackAs" => (
            ["as early back as"],
            ["as far back as"],
            "Use `as far back as` for referring to a time in the past.",
            "Corrects nonstandard `as early back as` to `as far back as`."
        ),
        "AsOfLate" => (
            ["as of lately"],
            ["as of late"],
            "The standard form is `as of late`.",
            "Corrects `as of lately` to `as of late`."
        ),
        "AsWell" => (
            ["aswell"],
            ["as well"],
            "`as well` should be written as two words.",
            "Corrects `aswell` to `as well`."
        ),
        "AtFaceValue" => (
            ["on face value"],
            ["at face value"],
            "`at face value is more idiomatic and more common.",
            "Corrects `on face value` to the more usual `at face value`."
        ),
        "AvoidAndAlso" => (
            ["and also"],
            ["and"],
            "Consider using just `and`.",
            "Reduces redundancy by replacing `and also` with `and`."
        ),
        // Avoid suggestions resulting in "a entire ...."
        "AWholeEntire" => (
            ["a whole entire"],
            ["a whole", "an entire"],
            "Avoid redundancy. Use either `whole` or `entire` for referring to the complete amount or extent.",
            "Corrects the redundancy in `whole entire` to `whole` or `entire`."
        ),
        "BadRap" => (
            ["bed rap", "bad rep"],
            ["bad rap"],
            "Did you mean `bad rap`?",
            "Changes `bed rap` to the proper idiom `bad rap`."
        ),
        "BandTogether" => (
            ["ban together"],
            ["band together"],
            "Did you mean `band together`?",
            "Detects and corrects the common error of using `ban together` instead of the idiom `band together`, which means to unite or join forces."
        ),
        "BareInMind" => (
            ["bare in mind"],
            ["bear in mind"],
            "Did you mean `bear in mind`?",
            "Ensures the phrase `bear in mind` is used correctly instead of `bare in mind`."
        ),
        "BatedBreath" => (
            ["baited breath"],
            ["bated breath"],
            "Did you mean `bated breath`?",
            "Changes `baited breath` to the correct `bated breath`."
        ),
        "BeckAndCall" => (
            ["back and call"],
            ["beck and call"],
            "Did you mean `beck and call`?",
            "Fixes `back and call` to `beck and call`."
        ),
        "BeenThere" => (
            ["bee there"],
            ["been there"],
            "Did you mean `been there`?",
            "Corrects the misspelling `bee there` to the proper phrase `been there`."
        ),
        "BestRegards" => (
            ["beat regards"],
            ["best regards"],
            "Use `best regards` to convey sincere well wishes in a closing.",
            "In valedictions, `best` expresses your highest regard—avoid the typo `beat regards`."
        ),
        "BlanketStatement" => (
            ["blanketed statement"],
            ["blanket statement"],
            "Use the more idiomatic phrasing.",
            "Corrects common errors in the phrase `blanket statement`."
        ),
        "Brutality" => (
            ["brutalness"],
            ["brutality"],
            "This word has a more standard, more common synonym.",
            "Suggests the more standard and common synonym `brutality`."
        ),
        "ByAccident" => (
            ["on accident"],
            ["by accident"],
            "Did you mean `by accident`?",
            "Incorrect preposition: `by accident` is the idiomatic expression."
        ),
        "CanBeSeen" => (
            ["can be seem"],
            ["can be seen"],
            "Did you mean `can be seen`?",
            "Corrects `can be seem` to the proper phrase `can be seen`."
        ),
        "CaseInPoint" => (
            ["case and point"],
            ["case in point"],
            "`Case in point` is the correct form of the phrase.",
            "Corrects `case and point` to `case in point`."
        ),
        "CaseSensitive" => (
            ["case sensitive"],
            ["case-sensitive"],
            "Use the hyphenated form for `case-sensitive`.",
            "Ensures `case-sensitive` is correctly hyphenated."
        ),
        "ChockFull" => (
            ["chock full"],
            ["chock-full"],
            "Use the hyphenated form for `chock-full`.",
            "Ensures `chock-full` is correctly hyphenated."
        ),
        "ClientSide" => (
            ["client's side"],
            ["client-side"],
            "In client-server contexts, use `client-side` rather than `client's side`.",
            "Corrects `client's side` to `client-side`, which is usual in `client-server contexts`."
        ),
        "CondenseAllThe" => (
            ["all of the"],
            ["all the"],
            "Consider simplifying to `all the`.",
            "Suggests removing `of` in `all of the` for a more concise phrase."
        ),
        "CoursingThroughVeins" => (
            ["cursing through veins"],
            ["coursing through veins"],
            "In this idiom, blood “courses” (flows) through veins, not “curses”.",
            "In English idioms, “to course” means to flow rapidly—so avoid the eggcorn `cursing through veins.`"
        ),
        "DampSquib" => (
            ["damp squid"],
            ["damp squib"],
            "Use the correct phrase for a disappointing outcome.",
            "Corrects the eggcorn `damp squid` to `damp squib`, ensuring the intended meaning of a failed or underwhelming outcome."
        ),
        "DayAndAge" => (
            ["day in age"],
            ["day and age"],
            "Use `day and age` for referring to the present time.",
            "Corrects the eggcorn `day in age` to `day and age`, which properly means the current era or time period."
        ),
        "DoNotWant" => (
            ["don't wan", "do not wan"],
            ["don't want", "do not want"],
            "Use the full verb “want” after negation: “don't want” or “do not want.”",
            "In English, negation still requires the complete verb form (“want”), so avoid truncating it to “wan.”"
        ),
        "EachAndEveryOne" => (
            ["each and everyone"],
            ["each and every one"],
            "Use `each and every one` for referring to a group of people or things.",
            "Corrects `each and everyone` to `each and every one`."
        ),
        "EludedTo" => (
            ["eluded to"],
            ["alluded to"],
            "Did you mean `alluded to`?",
            "Corrects `eluded to` to `alluded to` in contexts referring to indirect references."
        ),
        "EnMasse" => (
            ["on mass", "on masse", "in mass"],
            ["en masse"],
            "Did you mean `en masse`?",
            "Detects variants like `on mass` or `in mass` and suggests `en masse`."
        ),
        "EverPresent" => (
            ["ever present"],
            ["ever-present"],
            "Hyphenate `ever-present` when it functions as a compound adjective.",
            "Corrects the missing hyphen in `ever present` to the compound adjective `ever-present`."
        ),
        "Excellent" => (
            ["very good"],
            ["excellent"],
            "Vocabulary enhancement: use `excellent` instead of `very good`",
            "Provides a stronger word choice by replacing `very good` with `excellent` for clarity and emphasis."
        ),
        "ExpandBecause" => (
            ["cuz"],
            ["because"],
            "Use `because` instead of informal `cuz`",
            "Expands the informal abbreviation `cuz` to the full word `because` for formality."
        ),
        "ExpandDependencies" => (
            ["deps"],
            ["dependencies"],
            "Use `dependencies` instead of `deps`",
            "Expands the abbreviation `deps` to the full word `dependencies` for clarity."
        ),
        "ExpandDependency" => (
            ["dep"],
            ["dependency"],
            "Use `dependency` instead of `dep`",
            "Expands the abbreviation `dep` to the full word `dependency` for clarity."
        ),
        "ExpandMinimum" => (
            ["min"],
            ["minimum"],
            "Use `minimum` instead of `min`",
            "Expands the abbreviation `min` to the full word `minimum` for clarity."
        ),
        "ExpandStandardInput" => (
            ["stdin"],
            ["standard input"],
            "Use `standard input` instead of `stdin`",
            "Expands the abbreviation `stdin` to `standard input` for clarity."
        ),
        "ExpandStandardOutput" => (
            ["stdout"],
            ["standard output"],
            "Use `standard output` instead of `stdout`",
            "Expands the abbreviation `stdout` to `standard output` for clarity."
        ),
        "ExpandWith" => (
            ["w/"],
            ["with"],
            "Use `with` instead of `w/`",
            "Expands the abbreviation `w/` to the full word `with` for clarity."
        ),
        "ExpandWithout" => (
            ["w/o"],
            ["without"],
            "Use `without` instead of `w/o`",
            "Expands the abbreviation `w/o` to the full word `without` for clarity."
        ),
        "Expatriate" => (
            ["ex-patriot"],
            ["expatriate"],
            "Use the correct term for someone living abroad.",
            "Fixes the misinterpretation of `expatriate`, ensuring the correct term is used for individuals residing abroad."
        ),
        "FaceFirst" => (
            ["face first into"],
            ["face-first into"],
            "Should this be `face-first`?",
            "Ensures `face first` is correctly hyphenated as `face-first` when used before `into`."
        ),
        "FairBit" => (
            ["fare bit"],
            ["fair bit"],
            "A `decent amount` is a `fair bit`. `Fare` is the price of a ticket.",
            "Corrects malapropisms of `a fair bit`."
        ),
        "FarWorse" => (
            ["far worst"],
            ["far worse"],
            "Use `worse` for comparing. (`Worst` is for the extreme case)",
            "Corrects `far worst` to `far worse` for proper comparative usage."
        ),
        "FastPaste" => (
            ["fast paste", "fast-paste"],
            ["fast-paced"],
            "Did you mean `fast-paced`?",
            "Detects incorrect usage of `fast paste` or `fast-paste` and suggests `fast-paced` as the correct phrase."
        ),
        "FatalOutcome" => (
            ["fatal outcome"],
            ["death"],
            "Consider using `death` for clarity.",
            "Replaces `fatal outcome` with the more direct term `death` for conciseness."
        ),
        "FetalPosition" => (
            ["the feeble position"],
            ["the fetal position"],
            "Use the correct term for a curled-up posture.",
            "Ensures the correct use of `fetal position`, avoiding confusion with `feeble position`, which is not a standard phrase."
        ),
        "ForAllIntentsAndPurposes" => (
            ["for all intensive purposes"],
            ["for all intents and purposes"],
            "Use the correct phrase meaning 'in every practical sense'.",
            "Corrects `for all intensive purposes` to `for all intents and purposes`, ensuring the phrase conveys its intended meaning."
        ),
        "ForALongTime" => (
            ["for along time"],
            ["for a long time"],
            "Use the standard phrase `for a long time` to indicate an extended duration.",
            "Eliminates the incorrect merging in `for along time`."
        ),
        "ForAWhile" => (
            ["for while"],
            ["for a while"],
            "When describing a timeframe, use `a while`.",
            "Corrects the missing article in `for while` or `for awhile`, forming `for a while`."
        ),
        "FreeRein" => (
            ["free reign"],
            ["free rein"],
            "Use the correct phrase for unrestricted control.",
            "Ensures the correct use of `free rein`, avoiding confusion with `free reign`, which incorrectly suggests authority rather than freedom of action."
        ),
        "Freezing" => (
            ["very cold", "really cold", "extremely cold"],
            ["freezing"],
            "A more vivid adjective would better capture extreme cold.",
            "Encourages vivid writing by suggesting `freezing` instead of weaker expressions like `very cold.`"
        ),
        "GildedAge" => (
            ["guilded age"],
            ["Gilded Age"],
            "The period of economic prosperity is called the `Gilded Age`.",
            "If referring to the period of economic prosperity, the correct term is `Gilded Age`."
        ),
        "GoingTo" => (
            ["gong to"],
            ["going to"],
            "Did you mean `going to`?",
            "Corrects `gong to` to the intended phrase `going to`."
        ),
        "GuineaBissau" => (
            // Note: this lint matches any case but cannot correct wrong case
            // Note: It can only correct the hyphenation
            // Note: See linting/matcher.rs for case corrections
            // Note: $input must already be the correct case
            // Note: do not add other case variants here
            ["Guinea Bissau"],
            ["Guinea-Bissau"],
            "The official spelling is hyphenated.",
            "Checks for the correct official name of the African country."
        ),
        "HadOf" => (
            ["had of"],
            ["had have", "had've"],
            "Did you mean `had have` or `had've`?",
            "Flags the unnecessary use of `of` after `had` and suggests the correct forms."
        ),
        // "HadPassed" => (
        //     ["had past"],
        //     ["had passed"],
        //     "Did you mean the verb `passed`?",
        //     "Suggests `past` for `passed` in case a verb was intended."
        // ),
        "HalfAnHour" => (
            ["half an our"],
            ["half an hour"],
            "Remember the silent 'h' when writing `hour`: `half an hour`.",
            "Fixes the eggcorn `half an our` to the accepted `half an hour`."
        ),
        "Haphazard" => (
            ["half hazard", "half-hazard", "halfhazard"],
            ["haphazard"],
            "Use `haphazard` for randomness or lack of organization.",
            "Corrects the eggcorn `half hazard` to `haphazard`, which properly means lacking organization or being random."
        ),
        // "HasPassed" => (
        //     ["has past"],
        //     ["has passed"],
        //     "Did you mean the verb `passed`?",
        //     "Suggests `past` for `passed` in case a verb was intended."
        // ),
        // "HavePassed" => (
        //     ["have past"],
        //     ["have passed"],
        //     "Did you mean the verb `passed`?",
        //     "Suggests `past` for `passed` in case a verb was intended."
        // ),
        // "HavingPassed" => (
        //     ["having past"],
        //     ["having passed"],
        //     "Did you mean the verb `passed`?",
        //     "Suggests `past` for `passed` in case a verb was intended."
        // ),
        "HumanBeings" => (
            ["human's beings", "humans beings"],
            ["human beings"],
            "Use `human beings` to refer to people collectively.",
            "Eliminates the incorrect possessive/plural usage like `human's beings` or `humans beings`."
        ),
        "HumanLife" => (
            ["human live"],
            ["human life"],
            "Did you mean `human life`?",
            "Changes `human live` to `human life`."
        ),
        "HungerPang" => (
            ["hunger pain"],
            ["hunger pang"],
            "Did you mean `hunger pang`?",
            "Corrects `hunger pain` to `hunger pang`."
        ),
        "IAm" => (
            ["I a m"],
            ["I am"],
            "Did you mean `I am`?",
            "Fixes the incorrect spacing in `I a m` to properly form `I am`."
        ),
        "InAndOfItself" => (
            ["in of itself"],
            ["in and of itself"],
            "Use `in and of itself` for referring to something's inherent or intrinsic quality.",
            "Corrects nonstandard `in of itself` to standard `in and of itself`."
        ),
        "InAnyWay" => (
            ["in anyway"],
            ["in any way"],
            "Use `in any way` for emphasizing a point.",
            "Corrects ungrammatical `in anyway` to `in any way`."
        ),
        "InAWhile" => (
            ["in while"],
            ["in a while"],
            "When describing a timeframe, use `a while`.",
            "Corrects the missing article in `in while` or `in awhile`, forming `in a while`."
        ),
        "InCase" => (
            ["incase"],
            ["in case"],
            "`In case` should be written as two words.",
            "Corrects `incase` to `in case`."
        ),
        "InDetail" => (
            ["in details"],
            ["in detail"],
            "Use singular `in detail` for referring to a detailed description.",
            "Correct unidiomatic plural `in details` to `in detail`."
        ),
        "InMoreDetail" => (
            ["in more details"],
            ["in more detail"],
            "Use singular `in more detail` for referring to a detailed description.",
            "Correct unidiomatic plural `in more details` to `in more detail`."
        ),
        "InNeedOf" => (
            ["in need for"],
            ["in need of"],
            "Use `in need of` for when something is required or necessary.",
            "Corrects `in need for` to `in need of`."
        ),
        "InOneFellSwoop" => (
            ["in one foul swoop"],
            ["in one fell swoop"],
            "Use the correct phrase for something happening suddenly.",
            "Corrects `in one foul swoop` to `in one fell swoop`, preserving the phrase’s original meaning of sudden and complete action."
        ),
        "InsteadOf" => (
            ["in stead of"],
            ["instead of"],
            "Use the modern single word `instead of` to indicate a replacement.",
            "Corrects the archaic or mistaken separation `in stead of` to `instead of` in everyday usage."
        ),
        "Insurmountable" => (
            ["unsurmountable"],
            ["insurmountable"],
            "This word has a more standard, more common synonym.",
            "Suggests the more standard and common synonym `insurmountable`."
        ),
        "Intact" => (
            ["in tact"],
            ["intact"],
            "Use `intact` to mean undamaged or whole.",
            "Prevents the erroneous spacing in `in tact`; `intact` is the single correct word."
        ),
        "InThe" => (
            ["int he"],
            ["in the"],
            "Did you mean `in the`?",
            "Detects and corrects a spacing error where `in the` is mistakenly written as `int he`. Proper spacing is essential for readability and grammatical correctness in common phrases."
        ),
        "IsKnownFor" => (
            ["is know for"],
            ["is known for"],
            "Did you mean `is known for`?",
            "Typo: `known` is the correct past participle."
        ),
        "ItCan" => (
            ["It cam"],
            ["It can"],
            "Did you mean `It can`?",
            "Corrects the misspelling `It cam` to the proper phrase `It can`."
        ),
        "IveGotTo" => (
            ["I've go to"],
            ["I've got to"],
            "Use `I've got to` for necessity or obligation.",
            "Corrects the slip `I've go to` to the idiomatic `I've got to`."
        ),
        "JawDropping" => (
            ["jar-dropping"],
            ["jaw-dropping"],
            "Use the correct phrase for something astonishing.",
            "Corrects `jar-dropping` to `jaw-dropping`, ensuring the intended meaning of something that causes amazement."
        ),
        "JustDeserts" => (
            ["just desserts"],
            ["just deserts"],
            "Use the correct phrase for receiving what one deserves.",
            "Ensures `just deserts` is used correctly, preserving its meaning of receiving an appropriate outcome for one's actions."
        ),
        "KindRegards" => (
            ["kid regards"],
            ["kind regards"],
            "Did you mean `kind regards`?",
            "Changes `kid regards` to `kind regards`."
        ),
        "LastButNotLeast" => (
            ["last but not the least", "last, but not the least", "last but, not least", "last but not last"],
            ["last but not least"],
            "Use the more idiomatic phrasing.",
            "Corrects common errors in the phrase `last but not least`."
        ),
        "LastDitch" => (
            ["last ditch", "last ditched", "last-ditched"],
            ["last-ditch"],
            "In this idiom, `ditch` is a noun and a hyphen is needed.",
            "Corrects wrong variations of the idiomatic adjective `last-ditch`."
        ),
        "LetAlone" => (
            ["let along"],
            ["let alone"],
            "Did you mean `let alone`?",
            "Changes `let along` to `let alone`."
        ),
        "LikeThePlague" => (
            ["like a plague"],
            ["like the plague"],
            "`Things are avoided `like the plague` not `like a plague`.",
            "Corrects `like a plague` to `like the plague`."
        ),
        "Monumentous" => (
            ["monumentous"],
            ["momentous", "monumental"],
            "Retain `monumentous` for jocular effect. Otherwise `momentous` indicates great signifcance while `monumental` indicates imposing size.",
            "Advises using `momentous` or `monumental` instead of `monumentous` for serious usage."
        ),
        "MootPoint" => (
            ["mute point"],
            ["moot point"],
            "Did you mean `moot point`?",
            "Ensures `moot point` is used instead of `mute point`, as `moot` means debatable or irrelevant."
        ),
        "MuchWorse" => (
            ["much worst"],
            ["much worse"],
            "Use `worse` for comparing. (`Worst` is for the extreme case)",
            "Corrects `much worst` to `much worse` for proper comparative usage."
        ),
        "MyHouse" => (
            ["mu house"],
            ["my house"],
            "Did you mean `my house`?",
            "Fixes the typo `mu house` to `my house`."
        ),
        "NeedHelp" => (
            ["ned help"],
            ["need help"],
            "Did you mean `need help`?",
            "Changes `ned help` to the correct `need help`."
        ),
        "NerveRacking" => (
            ["nerve racking", "nerve wracking", "nerve wrecking", "nerve-wracking", "nerve-wrecking"],
            ["nerve-racking"],
            "Use `nerve-racking` for something that causes anxiety or tension.",
            "Corrects common misspellings and missing hyphen in `nerve-racking`."
        ),
        "NotIn" => (
            ["no in"],
            ["not in"],
            "Use `not in` for correct grammar.",
            "Replaces `no in` with `not in`."
        ),
        "NotTo" => (
            ["no to"],
            ["not to"],
            "Did you mean `not to`?",
            "Corrects `no to` to `not to`, ensuring proper negation."
        ),
        "OfCourse" => (
            ["off course", "o course"],
            ["Of course"],
            "Did you mean `of course`?",
            "Detects the non‐idiomatic phrase `off course` and suggests the correct form `of course`."
        ),
        "OffTheCuff" => (
            ["off the cuff"],
            ["off-the-cuff"],
            "Use the hyphenated form for `off-the-cuff`.",
            "Ensures `off-the-cuff` is correctly hyphenated."
        ),
        "OldWivesTale" => (
            ["old wise tale"],
            ["old wives' tale"],
            "Use the correct phrase for a superstition or myth.",
            "Corrects `old wise tale` to `old wives' tale`, preserving the phrase’s meaning as an unfounded traditional belief."
        ),
        "OnceInAWhile" => (
            ["once a while", "once and a while"],
            ["once in a while"],
            "The correct idiom is `once in a while`.",
            "Corrects two common malapropisms of `once in a while`."
        ),
        "OnSecondThought" => (
            ["on second though"],
            ["on second thought"],
            "Idiomatic expression: use `on second thought` instead of `on second though`",
            "Replaces the nonstandard `on second though` with the common idiom `on second thought` to indicate reconsideration."
        ),
        "OnTheSpurOfTheMoment" => (
            ["on the spurt of the moment"],
            ["on the spur of the moment"],
            "Use the correct phrase for acting spontaneously.",
            "Ensures the correct use of `on the spur of the moment`, avoiding confusion with the incorrect `spurt` variation."
        ),
        "PointIsMoot" => (
            ["your point is mute"],
            ["your point is moot"],
            "Did you mean `your point is moot`?",
            "Typo: `moot` (meaning debatable) is correct rather than `mute`."
        ),
        "PointsOfView" => (
            ["point of views"],
            ["points of view"],
            "The correct plural is `points of view`.",
            "Corrects pluralizing the wrong noun in `point of view`."
        ),
        "PortAuPrince" => (
            // Note: this lint matches any case but cannot correct wrong case
            // Note: It can only correct the hyphenation
            // Note: See linting/matcher.rs for case corrections
            // Note: $input must already be the correct case
            // Note: do not add other case variants here
            ["Port au Prince"],
            ["Port-au-Prince"],
            "The official spelling is hyphenated.",
            "Checks for the correct official name of the capital of Haiti."
        ),
        "PortoNovo" => (
            // Note: this lint matches any case but cannot correct wrong case
            // Note: It can only correct the hyphenation
            // Note: See linting/matcher.rs for case corrections
            // Note: $input must already be the correct case
            // Note: do not add other case variants here
            ["Porto Novo"],
            ["Porto-Novo"],
            "The official spelling is hyphenated.",
            "Checks for the correct official name of the capital of Benin."
        ),
        "PrayingMantis" => (
            ["preying mantis"],
            ["praying mantis"],
            "Use the insect's correct name.",
            "Corrects `preying mantis` to `praying mantis`, ensuring accurate reference to the insect’s characteristic pose."
        ),
        "RapidFire" => (
            ["rapid fire"],
            ["rapid-fire"],
            "It is more idiomatic to hypenate `rapid-fire`.",
            "Checks to ensure writers hyphenate `rapid-fire`."
        ),
        "RealTrouper" => (
            ["real trooper"],
            ["real trouper"],
            "Use the correct phrase for someone who perseveres.",
            "Ensures the correct use of `real trouper`, distinguishing it from `trooper`, which refers to a soldier or police officer."
        ),
        "RifeWith" => (
            ["ripe with"],
            ["rife with"],
            "Use the correct phrase for something abundant.",
            "Corrects `ripe with` to `rife with`, preserving the phrase’s meaning of being filled with something, often undesirable."
        ),
        "RoadMap" => (
            ["roadmap"],
            ["road map"],
            "Did you mean `road map`?",
            "Detects when `roadmap` is used instead of `road map`, prompting the correct spacing."
        ),
        "SameAs" => (
            ["same then"],
            ["same as"],
            "Did you mean `same as`?",
            "Corrects the incorrect phrase `same then` to the standard `same as`."
        ),
        "ScantilyClad" => (
            ["scandally clad"],
            ["scantily clad"],
            "Use the correct phrase for minimal attire.",
            "Fixes `scandally clad` to `scantily clad`, ensuring clarity in describing minimal attire."
        ),
        "ServerSide" => (
            ["server's side"],
            ["server-side"],
            "In client-server contexts, use `server-side` rather than `server's side`.",
            "Corrects `server's side` to `server-side`, which is usual in `client-server contexts`."
        ),
        "SimpleGrammatical" => (
            ["simply grammatical"],
            ["simple grammatical"],
            "Use `simple grammatical` for correct adjective usage.",
            "Corrects `simply grammatical` to `simple grammatical` for proper adjective usage."
        ),
        "SneakingSuspicion" => (
            ["sneaky suspicion"],
            ["sneaking suspicion"],
            "Did you mean `sneaking suspicion`?",
            "Changes `sneaky suspicion` to `sneaking suspicion`."
        ),
        "SoonerOrLater" => (
            ["sooner than later"],
            ["sooner rather than later", "sooner or later"],
            "Did you mean `sooner rather than later` or `sooner or later`?",
            "Fixes the improper phrase `sooner than later` by suggesting standard alternatives."
        ),
        "SpecialAttention" => (
            ["spacial attention"],
            ["special attention"],
            "Did you mean `special attention`?",
            "Changes `spacial attention` to `special attention`."
        ),
        "SpokeTooSoon" => (
            ["spoke to soon"],
            ["spoke too soon"],
            "Use the adverb `too` instead.",
            "Identifies common misuse of the preposition `to` in the phrase `spoke too soon`."
        ),
        "Starving" => (
            ["very hungry", "really hungry", "extremely hungry"],
            ["starving"],
            "A more vivid adjective would better convey intense hunger.",
            "Encourages vivid writing by suggesting `starving` instead of weaker expressions like `very hungry.`"
        ),
        "StateOfTheArt" => (
            ["state of art"],
            ["state of the art"],
            "Did you mean `state of the art`?",
            "Detects incorrect usage of `state of art` and suggests `state of the art` as the correct phrase."
        ),
        "SufficeItToSay" => (
            ["suffice to say"],
            ["suffice it to say"],
            "`Suffice it to say` is the more standard and more common variant.",
            "Corrects `suffice to say` to `suffice it to say`."
        ),
        "SupposedTo" => (
            ["suppose to"],
            ["supposed to"],
            "Did you mean `supposed to`?",
            "Fixes `suppose to` to the correct `supposed to`."
        ),
        "TakeItPersonally" => (
            ["take it personal"],
            ["take it personally"],
            "The more standard, less colloquial form is `take it personally`.",
            "Corrects `take it personal` to `take it personally`."
        ),
        "TakeItSeriously" => (
            ["take it serious"],
            ["take it seriously"],
            "Did you mean `take it seriously`?",
            "Ensures the correct use of the adverb `seriously` instead of the adjective `serious` in phrases like `take it seriously`."
        ),
        "ThatChallenged" => (
            ["the challenged"],
            ["that challenged"],
            "Did you mean `that challenged`?",
            "Changes `the challenged` to `that challenged` to fix the misspelling."
        ),
        "ThatChallenged" => (
            ["the challenged"],
            ["that challenged"],
            "Use `that challenged` for correct relative clause.",
            "Corrects `the challenged` to `that challenged` for proper relative clause usage."
        ),
        "ThatThis" => (
            ["the this"],
            ["that this"],
            "Did you mean `that this`?",
            "Fixes `the this` to the correct phrase `that this`."
        ),
        "TheAnother" => (
            ["the another"],
            ["the other", "another"],
            "Use `the other` or `another`, not both.",
            "Corrects `the another`."
        ),
        "ThereIsAny" => (
            ["there any"],
            ["there is any"],
            "Insert `is` for correct grammar.",
            "Replaces `there any` with `there is any`."
        ),
        "ThoughtProcess" => (
            ["though process"],
            ["thought process"],
            "Did you mean `thought process`?",
            "Changes `though process` to `thought process`."
        ),
        "TickingTimeClock" => (
            ["ticking time clock"],
            ["ticking time bomb", "ticking clock"],
            "Use `ticking time bomb` for disastrous consequences, otherwise avoid redundancy with just `ticking clock`.",
            "Corrects `ticking time clock` to `ticking time bomb` for idiomatic urgency or `ticking clock` otherwise."
        ),
        "ToDoHyphen" => (
            ["todo"],
            ["to-do"],
            "Hyphenate `to-do`.",
            "Ensures `to-do` is correctly hyphenated."
        ),
        "ToTheMannerBorn" => (
            ["to the manor born"],
            ["to the manner born"],
            "Use the correct phrase for being naturally suited to something.",
            "Corrects `to the manor born` to `to the manner born`, ensuring the intended meaning of being naturally suited to a way of life."
        ),
        "Towards" => (
            ["to towards"],
            ["towards"],
            "Use `towards` without the preceding `to`.",
            "Removes redundant `to` before `towards`."
        ),
        "TrialAndError" => (
            ["trail and error"],
            ["trial and error"],
            "You misspelled `trial`.",
            "Corrects `trail` to `trial` in `trial and error`."
        ),
        "TurnForTheWorse" => (
            ["turn for the worst"],
            ["turn for the worse"],
            "Use `turn for the worse` for a negative change in circumstances. Avoid the incorrect `turn for the worst`.",
            "Corrects the nonstandard `turn for the worst` to the idiomatic `turn for the worse`, used to describe a situation that has deteriorated."
        ),
        "TurnItOff" => (
            ["turn it of", "turn i of"],
            ["turn it off"],
            "Did you mean `turn it off`?",
            "Fixes the mistake in the phrase `turn it off`."
        ),
        "Unless" => (
            ["unless if"],
            ["unless"],
            "Use `unless` on its own.",
            "Corrects `unless if` to `unless`."
        ),
        "WantBe" => (
            ["want be"],
            ["won't be", "want to be"],
            "Did you mean `won't be` or `want to be`?",
            "Detects incorrect usage of `want be` and suggests `won't be` or `want to be` based on context."
        ),
        "WaveFunction" => (
            ["wavefunction"],
            ["wave function"],
            "Did you mean `wave function`?",
            "Identifies the mistake of merging `wave` and `function` into one word. In quantum mechanics, a `wave function` (written as two words) describes the mathematical function that represents the quantum state of a particle or system. Correct usage is crucial for clear and accurate scientific communication."
        ),
        "WellBeing" => (
            ["wellbeing"],
            ["well-being"],
            "Use the hyphenated form for `well-being`.",
            "Ensures `well-being` is correctly hyphenated."
        ),
        "WellKept" => (
            ["highly-kept", "highly kept"],
            // There may be other good alternatives such as closely-guarded or tightly-held
            ["well-kept"],
            "`Highly-kept` is not standard. To describe secrets, `well-kept` is the most used phrase.",
            "Flags `highly-kept` and recommends `well-kept` as an alternative."
        ),
        "WhetYourAppetite" => (
            ["wet your appetite"],
            ["whet your appetite"],
            "Use the correct phrase for stimulating desire.",
            "Ensures `whet your appetite` is used correctly, distinguishing it from the incorrect `wet` variation."
        ),
        "WholeEntire" => (
            ["whole entire"],
            ["whole", "entire"],
            "Avoid redundancy. Use either `whole` or `entire` for referring to the complete amount or extent.",
            "Corrects the redundancy in `whole entire` to `whole` or `entire`."
        ),
        "WillContain" => (
            ["will contains"],
            ["will contain"],
            "Did you mean `will contain`?",
            "Incorrect verb form: `will` should be followed by the base form `contain`."
        ),
        "WorldWarII" => (
            ["world war 2", "world war ii", "world war ii", "world war ii", "world war ii"],
            ["World War II"],
            "Use the correct capitalization for `World War II`.",
            "Ensures `World War II` is correctly capitalized."
        ),
        "WorseAndWorse" => (
            ["worst and worst", "worse and worst", "worst and worse"],
            ["worse and worse"],
            "Use `worse` for comparing. (`Worst` is for the extreme case)",
            "Corrects `worst and worst` to `worse and worse` for proper comparative usage."
        ),
        "WorseCaseScenario" => (
            ["worse case scenario", "worse-case scenario", "worse-case-scenario"],
            ["worst-case scenario"],
            "Use `worst` for referring to the worst possible scenario. (`Worse` is for comparing)",
            "Corrects `worst-case scenario` when the hyphen is missing or `worse` is used instead of `worst`."
        ),
        "WorseThan" => (
            ["worst than"],
            ["worse than"],
            "Use `worse` for comparing. (`Worst` is for the extreme case)",
            "Corrects `worst than` to `worse than` for proper comparative usage."
        ),
        "WorstCaseScenario" => (
            ["worst case scenario", "worst-case-scenario"],
            ["worst-case scenario"],
            "Hyphenate `worst-case`.",
            "Corrects `worst-case scenario` when the hyphen is missing or `worse` is used instead of `worst`."
        ),
        "WorstEver" => (
            ["worse ever"],
            ["worst ever"],
            "Use `worst` for the extreme case. (`Worse` is for comparing)",
            "Corrects `worse ever` to `worst ever` for proper comparative usage."
        ),
    });

    group.set_all_rules_to(Some(true));

    group
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{
        assert_lint_count, assert_nth_suggestion_result, assert_suggestion_result,
    };

    use super::lint_group;

    // AfterAWhile
    #[test]
    fn correct_after_while() {
        assert_suggestion_result(
            "bromite Crashes on all sites after while.",
            lint_group(),
            "bromite Crashes on all sites after a while.",
        );
    }

    // ALongTime
    #[test]
    fn detect_a_long_time() {
        assert_suggestion_result("along time", lint_group(), "a long time");
    }

    #[test]
    fn detect_a_long_time_real_world() {
        assert_suggestion_result(
            "Fast refreshing is very slow had to wait along time for it to update.",
            lint_group(),
            "Fast refreshing is very slow had to wait a long time for it to update.",
        );
    }

    // ALotWorst
    #[test]
    fn detect_a_lot_worse_atomic() {
        assert_suggestion_result("a lot worst", lint_group(), "a lot worse");
    }

    #[test]
    fn detect_a_lot_worse_real_world() {
        assert_suggestion_result(
            "On a debug build, it's even a lot worst.",
            lint_group(),
            "On a debug build, it's even a lot worse.",
        );
    }

    // AlzheimersDisease

    // AnAnother
    #[test]
    fn correct_an_another() {
        assert_suggestion_result(
            "Render shader to use it as texture in an another shader.",
            lint_group(),
            "Render shader to use it as texture in another shader.",
        );
    }

    #[test]
    fn correct_a_another() {
        assert_suggestion_result(
            "Audit login is a another package for laravel framework.",
            lint_group(),
            "Audit login is another package for laravel framework.",
        );
    }

    // AndIn
    // AndTheLike
    // AnotherAn
    #[test]
    fn correct_another_an() {
        assert_suggestion_result(
            "Yet another an atomic deployment tool.",
            lint_group(),
            "Yet another atomic deployment tool.",
        );
    }

    // AnotherOnes

    #[test]
    fn issue_574() {
        assert_lint_count("run by one", lint_group(), 0);
    }

    // turn it off

    #[test]
    fn turn_it_off_clean_lower() {
        assert_lint_count("turn it off", lint_group(), 0);
    }

    #[test]
    fn turn_it_off_clean_upper() {
        assert_lint_count("Turn it off", lint_group(), 0);
    }

    #[test]
    fn of_confusion() {
        assert_suggestion_result("Turn it of", lint_group(), "Turn it off");
    }

    #[test]
    fn i_and_of_confusion() {
        assert_suggestion_result("Turn i of", lint_group(), "Turn it off");
    }

    #[test]
    fn off_course() {
        assert_suggestion_result(
            "Yes, off course we should do that.",
            lint_group(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn o_course() {
        assert_suggestion_result(
            "Yes, o course we should do that.",
            lint_group(),
            "Yes, of course we should do that.",
        );
    }

    #[test]
    fn bad_rep() {
        assert_suggestion_result("bad rep", lint_group(), "bad rap");
    }

    #[test]
    fn baited_breath() {
        assert_suggestion_result("baited breath", lint_group(), "bated breath");
    }

    #[test]
    fn hunger_pain() {
        assert_suggestion_result("hunger pain", lint_group(), "hunger pang");
    }

    #[test]
    fn in_mass() {
        assert_suggestion_result("in mass", lint_group(), "en masse");
    }

    #[test]
    fn let_along() {
        assert_suggestion_result("let along", lint_group(), "let alone");
    }

    #[test]
    fn sneaky_suspicion() {
        assert_suggestion_result("sneaky suspicion", lint_group(), "sneaking suspicion");
    }

    #[test]
    fn supposed_to() {
        assert_suggestion_result("suppose to", lint_group(), "supposed to");
    }

    #[test]
    fn spacial_attention() {
        assert_suggestion_result("spacial attention", lint_group(), "special attention");
    }

    #[test]
    fn now_on_hold() {
        assert_lint_count("Those are now on hold for month.", lint_group(), 0);
    }

    #[test]
    fn point_is_moot() {
        assert_suggestion_result("Your point is mute.", lint_group(), "Your point is moot.");
    }

    #[test]
    fn issue_777() {
        assert_suggestion_result(
            "Last but not the least, with VS2013 you can use Web Essentials 2013",
            lint_group(),
            "Last but not least, with VS2013 you can use Web Essentials 2013",
        );
    }

    #[test]
    fn issue_790() {
        assert_suggestion_result(
            "This seems like a blanketed statement and I have not found any info to back up whether PyJWT is affected.",
            lint_group(),
            "This seems like a blanket statement and I have not found any info to back up whether PyJWT is affected.",
        );
    }

    #[test]
    fn guinea_bissau_missing_hyphen_only() {
        assert_suggestion_result("Guinea Bissau", lint_group(), "Guinea-Bissau");
    }

    #[test]
    fn detect_nerve_wracking_hyphen() {
        assert_suggestion_result(
            "We've gone through several major changes / upgrades to atlantis, and it's always a little bit nerve-wracking because if we mess something up we ...",
            lint_group(),
            "We've gone through several major changes / upgrades to atlantis, and it's always a little bit nerve-racking because if we mess something up we ...",
        );
    }

    #[test]
    fn detect_nerve_wrecking_hyphen() {
        assert_suggestion_result(
            "The issue happens to me on a daily basis, and it is nerve-wrecking because I become unsure if I have actually saved the diagram, but every time ...",
            lint_group(),
            "The issue happens to me on a daily basis, and it is nerve-racking because I become unsure if I have actually saved the diagram, but every time ...",
        );
    }

    #[test]
    fn detect_nerve_wracking_no_hyphen() {
        assert_suggestion_result(
            "Very nerve wracking landing in an unfamiliar mountainous airport in dead of night with no radar to show surrounding terrain.",
            lint_group(),
            "Very nerve-racking landing in an unfamiliar mountainous airport in dead of night with no radar to show surrounding terrain.",
        );
    }

    #[test]
    fn detect_nerve_wrecking_no_hyphen() {
        assert_suggestion_result(
            "I appreciate any kind of help since this is kind of nerve wrecking.",
            lint_group(),
            "I appreciate any kind of help since this is kind of nerve-racking.",
        );
    }

    #[test]
    fn detect_nerve_racking_no_hyphen() {
        assert_suggestion_result(
            "It's nerve racking to think about it because I have code inside the callback that resolves the member and somehow I feel like it's so ..",
            lint_group(),
            "It's nerve-racking to think about it because I have code inside the callback that resolves the member and somehow I feel like it's so ..",
        );
    }

    #[test]
    fn detect_atomic_whole_entire() {
        assert_suggestion_result("whole entire", lint_group(), "whole");
    }

    #[test]
    fn correct_atomic_a_whole_entire_to_a_whole() {
        assert_suggestion_result("a whole entire", lint_group(), "a whole");
    }

    #[test]
    fn correct_atomic_a_whole_entire_to_an_entire() {
        assert_nth_suggestion_result("a whole entire", lint_group(), "an entire", 1);
    }

    #[test]
    fn correct_real_world_whole_entire() {
        assert_suggestion_result(
            "[FR] support use system dns in whole entire app",
            lint_group(),
            "[FR] support use system dns in whole app",
        );
    }

    #[test]
    fn correct_real_world_a_whole_entire_to_a_whole() {
        assert_suggestion_result(
            "Start mapping a whole entire new planet using NASA’s MOLA.",
            lint_group(),
            "Start mapping a whole new planet using NASA’s MOLA.",
        );
    }

    #[test]
    fn correct_real_world_a_whole_entire_to_an_entire() {
        assert_nth_suggestion_result(
            "I am not sure I can pass in a whole entire query via the include.",
            lint_group(),
            "I am not sure I can pass in an entire query via the include.",
            1,
        );
    }

    fn in_detail_atomic() {
        assert_suggestion_result("in details", lint_group(), "in detail");
    }

    #[test]
    fn in_more_detail_atomic() {
        assert_suggestion_result("in more details", lint_group(), "in more detail");
    }

    #[test]
    fn in_detail_real_world() {
        assert_suggestion_result(
            "c++ - who can tell me \"*this pointer\" in details?",
            lint_group(),
            "c++ - who can tell me \"*this pointer\" in detail?",
        )
    }

    #[test]
    fn suggests_ticking_time_bomb() {
        assert_suggestion_result(
            "One element that can help up the stakes (and tension!) is a “ticking time clock.”",
            lint_group(),
            "One element that can help up the stakes (and tension!) is a “ticking time bomb.”",
        );
    }

    #[test]
    fn in_more_detail_real_world() {
        assert_suggestion_result(
            "Document the interface in more details · Issue #3 · owlbarn ...",
            lint_group(),
            "Document the interface in more detail · Issue #3 · owlbarn ...",
        );
    }

    #[test]
    fn detect_atomic_in_of_itself() {
        assert_suggestion_result("in of itself", lint_group(), "in and of itself");
    }

    #[test]
    fn correct_real_world_in_of_itself() {
        assert_suggestion_result(
            "This is not entirely unexpected in of itself, as Git and GitHub Desktop both generally prove fairly bad at delineating context intelligently...",
            lint_group(),
            "This is not entirely unexpected in and of itself, as Git and GitHub Desktop both generally prove fairly bad at delineating context intelligently...",
        )
    }

    #[test]
    fn suggests_ticking_clock() {
        assert_nth_suggestion_result(
            "The opportunity itself has a ticking time clock as all great opportunities do.",
            lint_group(),
            "The opportunity itself has a ticking clock as all great opportunities do.",
            1,
        );
    }

    #[test]
    fn detect_far_worse_atomic() {
        assert_suggestion_result("far worst", lint_group(), "far worse");
    }

    #[test]
    fn detect_far_worse_real_world() {
        assert_suggestion_result(
            "I mainly use Firefox (personal preference) and have noticed it has far worst performance than Chrome",
            lint_group(),
            "I mainly use Firefox (personal preference) and have noticed it has far worse performance than Chrome",
        );
    }

    #[test]
    fn detect_much_worse_atomic() {
        assert_suggestion_result("much worst", lint_group(), "much worse");
    }

    #[test]
    fn detect_much_worse_real_world() {
        assert_suggestion_result(
            "the generated image quality is much worst (actually nearly broken)",
            lint_group(),
            "the generated image quality is much worse (actually nearly broken)",
        );
    }

    #[test]
    fn detect_turn_for_the_worse_atomic() {
        assert_suggestion_result("turn for the worst", lint_group(), "turn for the worse");
    }

    #[test]
    fn detect_turn_for_the_worse_real_world() {
        assert_suggestion_result(
            "Very surprised to see this repo take such a turn for the worst.",
            lint_group(),
            "Very surprised to see this repo take such a turn for the worse.",
        );
    }

    #[test]
    fn detect_worst_and_worst_atomic() {
        assert_suggestion_result("worst and worst", lint_group(), "worse and worse");
    }

    #[test]
    fn detect_worst_and_worst_real_world() {
        assert_suggestion_result(
            "This control-L trick does not work for me. The padding is getting worst and worst.",
            lint_group(),
            "This control-L trick does not work for me. The padding is getting worse and worse.",
        );
    }

    #[test]
    fn detect_worse_and_worst_real_world() {
        assert_suggestion_result(
            "This progressively got worse and worst to the point that the machine (LEAD 1010) stopped moving alltogether.",
            lint_group(),
            "This progressively got worse and worse to the point that the machine (LEAD 1010) stopped moving alltogether.",
        );
    }

    #[test]
    fn detect_worse_than_atomic() {
        assert_suggestion_result("worst than", lint_group(), "worse than");
    }

    #[test]
    fn detect_worse_than_real_world() {
        assert_suggestion_result(
            "Project real image - inversion quality is worst than in StyleGAN2",
            lint_group(),
            "Project real image - inversion quality is worse than in StyleGAN2",
        );
    }

    #[test]
    fn detect_worst_ever_atomic() {
        assert_suggestion_result("worse ever", lint_group(), "worst ever");
    }

    #[test]
    fn detect_worst_ever_real_world() {
        assert_suggestion_result(
            "The Bcl package family is one of the worse ever published by Microsoft.",
            lint_group(),
            "The Bcl package family is one of the worst ever published by Microsoft.",
        );
    }

    #[test]
    fn detect_monumentous_atomic() {
        assert_suggestion_result("monumentous", lint_group(), "momentous");
    }

    #[test]
    fn detect_monumentous_real_world() {
        assert_suggestion_result(
            "I think that would be a monumentous step in the right direction, and would DEFINATLY turn heads in not just the music industry, but every ...",
            lint_group(),
            "I think that would be a momentous step in the right direction, and would DEFINATLY turn heads in not just the music industry, but every ...",
        );
    }

    #[test]
    fn detect_in_anyway_atomic() {
        assert_suggestion_result("in anyway", lint_group(), "in any way");
    }

    #[test]
    fn detect_in_anyway_real_world() {
        assert_suggestion_result(
            "The names should not affect your application in anyway and you can override extension names.",
            lint_group(),
            "The names should not affect your application in any way and you can override extension names.",
        );
    }
    #[test]
    fn detect_as_early_back_as() {
        assert_suggestion_result("as early back as", lint_group(), "as far back as");
    }

    #[test]
    fn detect_as_early_back_as_real_world() {
        assert_suggestion_result(
            "skin overrides also supports a wide variety of minecraft versions - as early back as 1.14.4.",
            lint_group(),
            "skin overrides also supports a wide variety of minecraft versions - as far back as 1.14.4.",
        );
    }

    #[test]
    fn detect_each_and_everyone() {
        assert_suggestion_result("each and everyone", lint_group(), "each and every one");
    }

    #[test]
    fn detect_each_and_everyone_real_world() {
        assert_suggestion_result(
            "I have modified each and everyone of them to keep only the best of the best!",
            lint_group(),
            "I have modified each and every one of them to keep only the best of the best!",
        );
    }

    #[test]
    fn test_instead_of() {
        assert_suggestion_result(
            "He used water in stead of soda.",
            lint_group(),
            "He used water instead of soda.",
        );
    }

    #[test]
    fn test_instead_of_clean() {
        // Ensure no lint is triggered when it's already correct
        assert_lint_count("He used water instead of soda.", lint_group(), 0);
    }

    #[test]
    fn test_intact() {
        assert_suggestion_result(
            "The code remains in tact after the merge.",
            lint_group(),
            "The code remains intact after the merge.",
        );
    }

    #[test]
    fn test_intact_clean() {
        assert_lint_count("The data set remains intact.", lint_group(), 0);
    }

    #[test]
    fn test_ive_got_to() {
        assert_suggestion_result(
            "I've go to finish this before Monday.",
            lint_group(),
            "I've got to finish this before Monday.",
        );
    }

    #[test]
    fn test_for_a_long_time() {
        assert_suggestion_result(
            "I was stuck there for along time.",
            lint_group(),
            "I was stuck there for a long time.",
        );
    }

    #[test]
    fn correct_another_ones() {
        assert_nth_suggestion_result(
            "Change list params of a resource, another ones change too",
            lint_group(),
            "Change list params of a resource, other ones change too",
            2,
        );
    }

    #[test]
    fn test_in_a_while() {
        assert_suggestion_result(
            "We’ll talk again in while.",
            lint_group(),
            "We’ll talk again in a while.",
        );
    }

    #[test]
    fn correct_another_things() {
        assert_nth_suggestion_result(
            "Another things to fix in the Mask editor",
            lint_group(),
            "Other things to fix in the Mask editor",
            1,
        );
    }

    #[test]
    fn test_human_beings() {
        assert_suggestion_result(
            "All humans beings deserve empathy.",
            lint_group(),
            "All human beings deserve empathy.",
        );
        assert_suggestion_result(
            "We should respect a human's beings fundamental rights.",
            lint_group(),
            "We should respect a human beings fundamental rights.",
        );
    }

    #[test]
    fn test_last_but_not_least() {
        assert_suggestion_result(
            "Last but not last, I'd like to thank my parents.",
            lint_group(),
            "Last but not least, I'd like to thank my parents.",
        );
    }

    #[test]
    fn test_half_an_hour() {
        assert_suggestion_result(
            "It took half an our to get there.",
            lint_group(),
            "It took half an hour to get there.",
        );
    }

    #[test]
    fn correct_the_another() {
        assert_suggestion_result(
            "Another possible cause is simply that the application does not have file creation permissions on the another machine.",
            lint_group(),
            "Another possible cause is simply that the application does not have file creation permissions on the other machine.",
        );
    }

    #[test]
    fn on_second_thought_clean() {
        assert_lint_count(
            "She considered driving home, but on second thought, she decided to walk.",
            lint_group(),
            0,
        );
    }

    #[test]
    fn on_second_thought_incorrect() {
        assert_suggestion_result(
            "I was going to buy it, but on second though, maybe I'll wait.",
            lint_group(),
            "I was going to buy it, but on second thought, maybe I'll wait.",
        );
    }

    #[test]
    fn correct_highly_kept_space() {
        assert_suggestion_result(
            "I assure you that frequency/angle dependence is a highly kept secret.",
            lint_group(),
            "I assure you that frequency/angle dependence is a well-kept secret.",
        );
    }

    #[test]
    fn expand_cuz() {
        assert_suggestion_result(
            "Stick around cuz I got a surprise for you at the end.",
            lint_group(),
            "Stick around because I got a surprise for you at the end.",
        );
    }

    #[test]
    fn on_second_thought_no_false_positive() {
        assert_lint_count(
            "My second though is that I'd prefer something else entirely.",
            lint_group(),
            0,
        );
    }

    #[test]
    fn excellent_clean() {
        assert_lint_count(
            "The performance was excellent, drawing praise from all critics.",
            lint_group(),
            0,
        );
    }

    #[test]
    fn excellent_incorrect() {
        assert_suggestion_result(
            "Her results were very good this semester.",
            lint_group(),
            "Her results were excellent this semester.",
        );
    }

    #[test]
    fn excellent_no_false_positive() {
        assert_lint_count(
            "He radiated a sense of very goodness in his charitable acts.",
            lint_group(),
            0,
        );
    }

    #[test]
    fn correct_highly_kept_no_hyphen() {
        assert_suggestion_result(
            "Well, Kushina's giving birth was already a highly-kept secret so it makes sense to operate with only the completely necessary personnel.",
            lint_group(),
            "Well, Kushina's giving birth was already a well-kept secret so it makes sense to operate with only the completely necessary personnel.",
        );
    }

    #[test]
    fn correct_on_face_value() {
        assert_suggestion_result(
            "Obviously what you want is possible and on face value it's a trivial change on our end.",
            lint_group(),
            "Obviously what you want is possible and at face value it's a trivial change on our end.",
        );
    }

    #[test]
    fn correct_trail_and_error() {
        assert_suggestion_result(
            "It was produced through trail and error.",
            lint_group(),
            "It was produced through trial and error.",
        );
    }

    #[test]
    fn correct_unless_if() {
        assert_suggestion_result(
            "Simplex does not interpret the following invite link as an invite link unless if it has https:// in front of it.",
            lint_group(),
            "Simplex does not interpret the following invite link as an invite link unless it has https:// in front of it.",
        );
    }

    #[test]
    fn suffice_it_to_say() {
        assert_suggestion_result(
            "I don't fully grok the bug, but suffice to say it is not an RCD issue.",
            lint_group(),
            "I don't fully grok the bug, but suffice it to say it is not an RCD issue.",
        );
    }

    #[test]
    fn correct_for_while() {
        assert_suggestion_result(
            "Build flutter releases in github actions for production only android for while.",
            lint_group(),
            "Build flutter releases in github actions for production only android for a while.",
        );
    }

    #[test]
    fn correct_like_a_plague() {
        assert_suggestion_result(
            "Below is the worst example of them all (avoid such coding like a plague):",
            lint_group(),
            "Below is the worst example of them all (avoid such coding like the plague):",
        );
    }

    #[test]
    fn correct_case_and_point_spaced() {
        assert_suggestion_result(
            "They are just not as high of a priority as other tasks that user's are requesting for, a case and point is I have never ran into this issue.",
            lint_group(),
            "They are just not as high of a priority as other tasks that user's are requesting for, a case in point is I have never ran into this issue.",
        );
    }

    #[test]
    fn correct_worse_case_space() {
        assert_suggestion_result(
            "In the worse case scenario, remote code execution could be achieved.",
            lint_group(),
            "In the worst-case scenario, remote code execution could be achieved.",
        );
    }

    #[test]
    fn correct_worse_case_hyphen() {
        assert_suggestion_result(
            "Basically I want my pods to get the original client IP address... or at least have X-Forwarded-For header, in a worse-case scenario.",
            lint_group(),
            "Basically I want my pods to get the original client IP address... or at least have X-Forwarded-For header, in a worst-case scenario.",
        );
    }

    #[test]
    fn correct_worse_case_two_hyphens() {
        assert_suggestion_result(
            "In a worse-case-scenario, the scenario class code and the results being analysed, become out of sync, and so the wrong labels are applied.",
            lint_group(),
            "In a worst-case scenario, the scenario class code and the results being analysed, become out of sync, and so the wrong labels are applied.",
        );
    }

    #[test]
    fn correct_worst_case_space() {
        assert_suggestion_result(
            "The worst case scenario can be calculated without looking at streams of data.",
            lint_group(),
            "The worst-case scenario can be calculated without looking at streams of data.",
        );
    }

    #[test]
    fn correct_worst_case_two_hyphens() {
        assert_suggestion_result(
            "CAPD worst-case-scenario cloud simulator for naughty clouds.",
            lint_group(),
            "CAPD worst-case scenario cloud simulator for naughty clouds.",
        );
    }

    #[test]
    fn corrects_dont_wan() {
        assert_suggestion_result(
            "I don't wan to pay for this.",
            lint_group(),
            "I don't want to pay for this.",
        );
    }

    #[test]
    fn correct_clients_side() {
        assert_suggestion_result(
            "I want to debug this server-side as I cannot find out why the connection is being refused from the client's side.",
            lint_group(),
            "I want to debug this server-side as I cannot find out why the connection is being refused from the client-side.",
        );
    }

    #[test]
    fn corrects_mixed_case() {
        assert_suggestion_result(
            "Don't Wan that option.",
            lint_group(),
            "Don't Want that option.",
        );
    }

    #[test]
    fn does_not_flag_already_correct() {
        assert_lint_count("I don't want to leave.", lint_group(), 0);
    }

    #[test]
    fn detect_cursing_through_veins_atomic() {
        assert_suggestion_result(
            "cursing through veins",
            lint_group(),
            "coursing through veins",
        );
    }

    #[test]
    fn detect_cursing_through_veins_real_world() {
        assert_suggestion_result(
            "It felt like the drugs were cursing through veins.",
            lint_group(),
            "It felt like the drugs were coursing through veins.",
        );
    }

    #[test]
    fn does_not_flag_other_contexts() {
        assert_lint_count(
            "He was cursing through the entire meeting.",
            lint_group(),
            0,
        );
    }

    #[test]
    fn correct_servers_side() {
        assert_suggestion_result(
            "A client-server model where the client can execute commands in a terminal on the server's side",
            lint_group(),
            "A client-server model where the client can execute commands in a terminal on the server-side",
        );
    }

    #[test]
    fn correct_last_ditched() {
        assert_suggestion_result(
            "I was actually just trying that as a last ditched attempt to get it working, previously those ...",
            lint_group(),
            "I was actually just trying that as a last-ditch attempt to get it working, previously those ...",
        );
    }

    #[test]
    fn correct_last_ditch_space() {
        assert_suggestion_result(
            "There are unique use cases and is meant to be a last ditch option.",
            lint_group(),
            "There are unique use cases and is meant to be a last-ditch option.",
        );
    }

    #[test]
    fn corrects_once_a_while() {
        assert_suggestion_result(
            "For me it is a SMB mount I have on the client device that I sync only once a while for a backup into the cloud.",
            lint_group(),
            "For me it is a SMB mount I have on the client device that I sync only once in a while for a backup into the cloud.",
        );
    }

    #[test]
    #[ignore = "There's a bug when changing the length of title case phrases.\nI believe there's a fix coming in a PR. Uncomment when fixed."]
    fn corrects_gilded_age_capitalized() {
        assert_suggestion_result(
            "It is especially a reflection of the socio-economic patterns in the Guilded Age.",
            lint_group(),
            "It is especially a reflection of the socio-economic patterns in the Gilded Age.",
        );
    }

    #[test]
    #[ignore = "Currently the correct spelling is suggested but the case is not changed.\nThis may also be fixed in the coming PR mentioned above."]
    fn corrects_gilded_age_lowercase() {
        assert_suggestion_result(
            "It is especially a reflection of the socio-economic patterns in the guilded age.",
            lint_group(),
            "It is especially a reflection of the socio-economic patterns in the Gilded Age.",
        );
    }

    #[test]
    fn corrects_once_and_a_while() {
        assert_suggestion_result(
            "Every once and a while all the links on my page seem to stop working.",
            lint_group(),
            "Every once in a while all the links on my page seem to stop working.",
        );
    }

    #[test]
    fn detect_ever_present_atomic() {
        assert_suggestion_result("ever present", lint_group(), "ever-present");
    }

    #[test]
    fn detect_ever_present_real_world() {
        assert_suggestion_result(
            "Distrust was an ever present tension in the negotiations.",
            lint_group(),
            "Distrust was an ever-present tension in the negotiations.",
        );
    }

    #[test]
    fn corrects_fair_bit() {
        assert_suggestion_result(
            "I've read through a fare bit of the ecosystem framework, but I am not clear on what is modified...",
            lint_group(),
            "I've read through a fair bit of the ecosystem framework, but I am not clear on what is modified...",
        );
    }

    #[test]
    fn corrects_take_it_personal() {
        assert_suggestion_result(
            "This is not personal, do not take it personal, we also think Thingsboard is a extraordinary tool (we are using in several scenarios in fact)",
            lint_group(),
            "This is not personal, do not take it personally, we also think Thingsboard is a extraordinary tool (we are using in several scenarios in fact)",
        );
    }

    #[test]
    fn corrects_as_of_lately() {
        assert_suggestion_result(
            "I haven't noticed any crashing with AMDGPU as of lately, so this looks to not be an issue anymore.",
            lint_group(),
            "I haven't noticed any crashing with AMDGPU as of late, so this looks to not be an issue anymore.",
        )
    }

    #[test]
    fn corrects_points_of_view() {
        assert_suggestion_result(
            "This will produce a huge amount of raw data, representing the region in multiple point of views.",
            lint_group(),
            "This will produce a huge amount of raw data, representing the region in multiple points of view.",
        )
    }

    #[test]
    fn corrects_brutalness() {
        assert_suggestion_result(
            "the mildness and brutalness of the story rises.",
            lint_group(),
            "the mildness and brutality of the story rises.",
        )
    }

    #[test]
    fn corrects_unsurmountable() {
        assert_suggestion_result(
            "That being said, if you find upgrading to newer versions to be unsurmountable, please open an issue.",
            lint_group(),
            "That being said, if you find upgrading to newer versions to be insurmountable, please open an issue.",
        )
    }
}
