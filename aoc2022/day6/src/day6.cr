def first_all_different(signal : String, size : UInt32 ) : UInt32
  char_set = [] of Char
  counter = 1_u32;
  signal.chars.each do |char| 
    if char_set.size == size 
      char_set.shift
    end
    char_set << char
    if char_set.to_set.size == size
      break
    end
    counter += 1
  end
  counter
end

puts "\nPart One Tests\n"

puts first_all_different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)
puts first_all_different("nppdvjthqldpwncqszvftbrmjlhg", 4)
puts first_all_different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
puts first_all_different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)
puts "=========== Part One =========== "
puts first_all_different("vldvlvvhwhttcsttpntnvvqpqpddmlddwhwcwnwmwfwppdrrhllcwwgwhwvhwhshchshtsstzstztgtlggcwwtptrppfpggwpwmppnfpfnfznffmvfmvffgfwwwgrrqgglrgrqqlslhhgjgwwdswsbwwqswqwnqwqpqttqstqtjqqctcbtcbttcptpbbsmmhggwmggjllcpcvcrrphpjjwzwgzgtzggscggwdgghlhnhddlclrcchllrlbbnlbbgpbgghvggpbgpgssbszzjbbcpbpttwztwwgngnqqgllcvlvlzvzwwzssbppjwwvswwbjwbjwwwsjsdjsszmssfvsvcscqssnbnzbzzhsszttfvvdllzjzsjjzzvzsvzzdvvcpphfhzhrzrnnwffnrrhwrhrnrhnrrgfgmmzwmzmhzzsrzrgggnfntnpnqqdhhmrmrrqdqwqsqmssmllmvmgglttdmdffwzzhszhzphzphplpvpzvvsvcvgvqgqpgqqrmqmqssdbdcdpphddvppfggwrgrdgrggtvggrttmpmvmnmwnmmlcccwssfjftfbbgqbbnlldzlddzbbvmmvbmvvbsszggmnggqsqvssdlddqbbtfbttlhhlssplphlhzzcmmdcmmlqlqmmtddztzctzzlglwlzwwsmsddbrrzqqpdpjdpjjzhjjjzttvvwcvcdvvwqvvwqqgpgjgwgvgmmdjjmbbttmffwgwgppspzzvddjnjppmsscjczcqcczlzjzbbzmzllsshrrdtdttjdtdnnwbwttzptztrzzgfgfqfjjlwjjbfbsbmmhphqqjlqjjjshsvvrzrffvnfftrrpzzrdrbbcqbqjqccphhwfwgfwgwgwzzhggddctcsttjgjppghppfhhlnhhhwfwrfrggtjjgjbgbjbrrssjvvlcvvdzvdzzjhhgpprnpnpggfmfgfzfrzzsjjwhhrmmmvppgfffdsfddwlddmccjfcjjrwwsmwsstlsszbbjssjtsjjmpjpqqzffhphfffrzffnbnvnzvnvsvjjbljlflglrlvvghhcrhccnhccdbcdclcbbgffhwhnwwrwbbngnwwlpwlwsllsffzvzbzhbhppvdvfdvffvsvlslcslsqqszqzrrqbrrpjjvgjggbvvpbblnlvnnvrrprbpbvbccrjsdfrnqvhgfrwtvqjfzflnqqgbwlvfwwmrgnsltsqjfcctdwhqrstpvllhfrbnvnhvvgfvhbsgppslqnhmwdnrjnpfmrppppqpmrmcvtndrgwngrblpvrgnbhgtflthdjdtqhwcfzbdwsjshhnglprfcjfdwffmhbvhshbzsgdsbdwpcjfhrccqmjslqjjrwnbtqftqlvgnpmmzlfnmjzvrfslmmvhqwzjqbwsqfcnlmgdwlcbrlqlfwzhcfjsnncnnjqltfccmllhnjczqssjnjmhqbhqzdplbvfdpmdjmgthvqgjzqqschzgtmpdzmvvhlrwjpbqfplcqdbjfjfcrzdclctldvvqphtnvmgzvwprhzmbsbgfjlhvbtcmnccrgrpjlcjqqfcgjhtwvfstrtzszgcprmcngbhbdvjbfvgqdlhzgtzcjqnjmdtmwzchcfhzhgwprgrzbfwbhstfbprhnbzhsglmwhcjbppnshmzlnzsmbhcrmvpdgftfrwjfnmdvtrrqwjmzlbdjppsnvmlstsnrjwslqtqfmfcspzgrqhshhvclvqdfpbmzvsnthmcrzdmzqcjnnmbwbdlbfmcrzjjsrjbwchlvljplqdbjfchbslcvbjvvzfgdzmmnrgqwftppgpfwhfvdqqsrphrqmdtzjghlldfpgnczzjhqhfjvgqmcpzqssfqsfblcvqfttznpmvczprcptgcfwwlwsmqvfrjzcwbhppsjghmltqtcqljmpjzddncbslqdvgzhdvfpdpgpzljrfnjwdtnbdjwzjvmhqvnrdrjmhcfsbjcwtflljwjvtjbsbhzbghnzwtwpwzzwnfhwszsghggqthcbtwjrhdphbdslzmwhpmtjfbnpzspfqrvvtsjpvjmbtwrsqvfzzphllbvmvczsphdtblgdczjsqqthgscdqpvnmbpblspbcmpgjjtjtdrwhrqcgbrvlcsrwzwnjlgbfjbfgwpqpvnnmmgbdrhrwsptmddvtgbjhgzcphzmscjrqlnngpsnmjhvtmnhmqmcwdpjpbjsntcgppqrjndfsfhrhcvgcmrfrrsvnddwjsndlwffrgqnldqnvtgfvdrwtrlcqhltmbvdndzpdqndqrbtzqwbmbtzzsqftjftcnbrtsgvdrmnqlbbmhwmzpngltcslwdnpzpvstpdtfdcqvlwtbppsjvpdbspzlwnfvgcslzvmrpgplbnrvwpfwnrhcncdzptrjsqvghrczmrfwfntqlvlccwtbwqdcngzlqqvvnvfttqmcbqfqndhlsmbvcnstjcbpffmsptdnqbntnhhdctgdbnvwzgwznsgpvmslpmlcffsdtfnlcmbdgbntcslrlhfmrpddmjjttbtcrgbmfsbbwphtmlrdrgffdlnrwndhttjrplstwfwlpnljlcjphdvdvslwvnstqlrsgwdltlqwzdgsltzjqsgwqpbzgvqbdmvqtdgsnhttqprttzrnmddzdnqsgmnhfrmmfnrmltgjqpmgmdzdwpnzdsgstmwlgstmtjtqlhngmpwdqscrjmpmndddppsgthtbmznndswpsftcfltmhbbglnlmlszsssrlgbzcqpngvtgttlblsthmrltgctpgpjmhqqbldfgcrmclsmjhnjspzgwpmwncjgwrgdjmfgrpndztfdssmjhfmstgbjjzvmrpbmfzljpffwgvszwvtclfdnnbswzpjgthcbzpcbmgfvhfwnfthjdmnzdptqflzldnvnmfqnbggsrzjjssdntqhsjltjlfvjhncbrfbhllmqdpdnlbjptdcrqqghvvfvzzvtbvnjhshptcmgcnlmmpgdggdsfnpfsgrbcnfvrvfcqdfdfbgfvqcspvnrrljsgdtsbbcvcqnlwmrfgjhbdnjqgpggnpqtqgwspgljbrsggnmbmdbctwsdzqjmzrcwqgsvnhlpnlnqgfvgdfpgwsbqpqjcjpwwbrtzqhhgswggwnhmzwtvrqcrgbsrjspmczctgprgcwjtnzghvzqgnpmfqdswnzfzdbpdnpcgmwdrpmjtzhhwcnrrpfvqsgbrzngmwdgvjnmcftcjpcmdvcwjgfgvjrblsdnbsgfmjnvvvfwpfhztbgfddlcljzmwrwglnrfbnphlpvvcbfnwpsmsnhshzwrpnljzstmglrwdcctqlbwvqtfpjjdqzgncwgsnhczhvnvnzzmsjhfvgpdrhsmgnrfjcrzblzncdlffjrcqrqqdnjhgwgdgjlgbtphzgrjvmgnqrvdgmtgvpdbzqcvsfctrntjsnfcwnzpslzsvmtjhtcvcdsbqflbmtvclssrvcwjwcbspjgqhznzpttlbsnrpdrnqsddlcvjdrnqcgjhjfdclhwscdpbsjzcmrhgfwdnqbzqjsdqwbqdfcrztzfvclvbnhqstjztqdmsvlfqzmllphgrnwjqpdlrdnsbdltrggvlmdpfdwrdwnsdnscfcjzmrwqpsjwzrqcqcfndlvtfftbhdcbgcgtrfqrqnvwgbpstrtzgpjcjswfcwgvfnwlpprthntmlqbchjcwnqgppchgtvrzjbdzptfqqbmchmpqlnnfvpghjnmshpcjgsprbjpqnpwwddnswnfjvbzwszplhnhgzmzdqncmqrqtbqhdwbtrbrljpcwbdhjzvcqpdgvbtczlhwwzfmgvffnbcglpdqjdsnhhtnvvmtnhlbqcfqjfcgmcnqstzbgmqcfsgrzncwcfrlpsctpspvvdzwtbrhqzhfcwvwqvtzrmfjgpmlsdjmlgwhcldqlhsjvprvnmzcsldzfpzhmzdbqbpwnrsffswbjcwjgblnlcwzqlmcgfstqggdbsqpcpqcgfvn", 4)



puts "\nPart Two Tests\n"

puts first_all_different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
puts first_all_different("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)
puts first_all_different("nppdvjthqldpwncqszvftbrmjlhg", 14)
puts first_all_different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
puts first_all_different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)

puts "=========== Part Two =========== "
puts first_all_different("vldvlvvhwhttcsttpntnvvqpqpddmlddwhwcwnwmwfwppdrrhllcwwgwhwvhwhshchshtsstzstztgtlggcwwtptrppfpggwpwmppnfpfnfznffmvfmvffgfwwwgrrqgglrgrqqlslhhgjgwwdswsbwwqswqwnqwqpqttqstqtjqqctcbtcbttcptpbbsmmhggwmggjllcpcvcrrphpjjwzwgzgtzggscggwdgghlhnhddlclrcchllrlbbnlbbgpbgghvggpbgpgssbszzjbbcpbpttwztwwgngnqqgllcvlvlzvzwwzssbppjwwvswwbjwbjwwwsjsdjsszmssfvsvcscqssnbnzbzzhsszttfvvdllzjzsjjzzvzsvzzdvvcpphfhzhrzrnnwffnrrhwrhrnrhnrrgfgmmzwmzmhzzsrzrgggnfntnpnqqdhhmrmrrqdqwqsqmssmllmvmgglttdmdffwzzhszhzphzphplpvpzvvsvcvgvqgqpgqqrmqmqssdbdcdpphddvppfggwrgrdgrggtvggrttmpmvmnmwnmmlcccwssfjftfbbgqbbnlldzlddzbbvmmvbmvvbsszggmnggqsqvssdlddqbbtfbttlhhlssplphlhzzcmmdcmmlqlqmmtddztzctzzlglwlzwwsmsddbrrzqqpdpjdpjjzhjjjzttvvwcvcdvvwqvvwqqgpgjgwgvgmmdjjmbbttmffwgwgppspzzvddjnjppmsscjczcqcczlzjzbbzmzllsshrrdtdttjdtdnnwbwttzptztrzzgfgfqfjjlwjjbfbsbmmhphqqjlqjjjshsvvrzrffvnfftrrpzzrdrbbcqbqjqccphhwfwgfwgwgwzzhggddctcsttjgjppghppfhhlnhhhwfwrfrggtjjgjbgbjbrrssjvvlcvvdzvdzzjhhgpprnpnpggfmfgfzfrzzsjjwhhrmmmvppgfffdsfddwlddmccjfcjjrwwsmwsstlsszbbjssjtsjjmpjpqqzffhphfffrzffnbnvnzvnvsvjjbljlflglrlvvghhcrhccnhccdbcdclcbbgffhwhnwwrwbbngnwwlpwlwsllsffzvzbzhbhppvdvfdvffvsvlslcslsqqszqzrrqbrrpjjvgjggbvvpbblnlvnnvrrprbpbvbccrjsdfrnqvhgfrwtvqjfzflnqqgbwlvfwwmrgnsltsqjfcctdwhqrstpvllhfrbnvnhvvgfvhbsgppslqnhmwdnrjnpfmrppppqpmrmcvtndrgwngrblpvrgnbhgtflthdjdtqhwcfzbdwsjshhnglprfcjfdwffmhbvhshbzsgdsbdwpcjfhrccqmjslqjjrwnbtqftqlvgnpmmzlfnmjzvrfslmmvhqwzjqbwsqfcnlmgdwlcbrlqlfwzhcfjsnncnnjqltfccmllhnjczqssjnjmhqbhqzdplbvfdpmdjmgthvqgjzqqschzgtmpdzmvvhlrwjpbqfplcqdbjfjfcrzdclctldvvqphtnvmgzvwprhzmbsbgfjlhvbtcmnccrgrpjlcjqqfcgjhtwvfstrtzszgcprmcngbhbdvjbfvgqdlhzgtzcjqnjmdtmwzchcfhzhgwprgrzbfwbhstfbprhnbzhsglmwhcjbppnshmzlnzsmbhcrmvpdgftfrwjfnmdvtrrqwjmzlbdjppsnvmlstsnrjwslqtqfmfcspzgrqhshhvclvqdfpbmzvsnthmcrzdmzqcjnnmbwbdlbfmcrzjjsrjbwchlvljplqdbjfchbslcvbjvvzfgdzmmnrgqwftppgpfwhfvdqqsrphrqmdtzjghlldfpgnczzjhqhfjvgqmcpzqssfqsfblcvqfttznpmvczprcptgcfwwlwsmqvfrjzcwbhppsjghmltqtcqljmpjzddncbslqdvgzhdvfpdpgpzljrfnjwdtnbdjwzjvmhqvnrdrjmhcfsbjcwtflljwjvtjbsbhzbghnzwtwpwzzwnfhwszsghggqthcbtwjrhdphbdslzmwhpmtjfbnpzspfqrvvtsjpvjmbtwrsqvfzzphllbvmvczsphdtblgdczjsqqthgscdqpvnmbpblspbcmpgjjtjtdrwhrqcgbrvlcsrwzwnjlgbfjbfgwpqpvnnmmgbdrhrwsptmddvtgbjhgzcphzmscjrqlnngpsnmjhvtmnhmqmcwdpjpbjsntcgppqrjndfsfhrhcvgcmrfrrsvnddwjsndlwffrgqnldqnvtgfvdrwtrlcqhltmbvdndzpdqndqrbtzqwbmbtzzsqftjftcnbrtsgvdrmnqlbbmhwmzpngltcslwdnpzpvstpdtfdcqvlwtbppsjvpdbspzlwnfvgcslzvmrpgplbnrvwpfwnrhcncdzptrjsqvghrczmrfwfntqlvlccwtbwqdcngzlqqvvnvfttqmcbqfqndhlsmbvcnstjcbpffmsptdnqbntnhhdctgdbnvwzgwznsgpvmslpmlcffsdtfnlcmbdgbntcslrlhfmrpddmjjttbtcrgbmfsbbwphtmlrdrgffdlnrwndhttjrplstwfwlpnljlcjphdvdvslwvnstqlrsgwdltlqwzdgsltzjqsgwqpbzgvqbdmvqtdgsnhttqprttzrnmddzdnqsgmnhfrmmfnrmltgjqpmgmdzdwpnzdsgstmwlgstmtjtqlhngmpwdqscrjmpmndddppsgthtbmznndswpsftcfltmhbbglnlmlszsssrlgbzcqpngvtgttlblsthmrltgctpgpjmhqqbldfgcrmclsmjhnjspzgwpmwncjgwrgdjmfgrpndztfdssmjhfmstgbjjzvmrpbmfzljpffwgvszwvtclfdnnbswzpjgthcbzpcbmgfvhfwnfthjdmnzdptqflzldnvnmfqnbggsrzjjssdntqhsjltjlfvjhncbrfbhllmqdpdnlbjptdcrqqghvvfvzzvtbvnjhshptcmgcnlmmpgdggdsfnpfsgrbcnfvrvfcqdfdfbgfvqcspvnrrljsgdtsbbcvcqnlwmrfgjhbdnjqgpggnpqtqgwspgljbrsggnmbmdbctwsdzqjmzrcwqgsvnhlpnlnqgfvgdfpgwsbqpqjcjpwwbrtzqhhgswggwnhmzwtvrqcrgbsrjspmczctgprgcwjtnzghvzqgnpmfqdswnzfzdbpdnpcgmwdrpmjtzhhwcnrrpfvqsgbrzngmwdgvjnmcftcjpcmdvcwjgfgvjrblsdnbsgfmjnvvvfwpfhztbgfddlcljzmwrwglnrfbnphlpvvcbfnwpsmsnhshzwrpnljzstmglrwdcctqlbwvqtfpjjdqzgncwgsnhczhvnvnzzmsjhfvgpdrhsmgnrfjcrzblzncdlffjrcqrqqdnjhgwgdgjlgbtphzgrjvmgnqrvdgmtgvpdbzqcvsfctrntjsnfcwnzpslzsvmtjhtcvcdsbqflbmtvclssrvcwjwcbspjgqhznzpttlbsnrpdrnqsddlcvjdrnqcgjhjfdclhwscdpbsjzcmrhgfwdnqbzqjsdqwbqdfcrztzfvclvbnhqstjztqdmsvlfqzmllphgrnwjqpdlrdnsbdltrggvlmdpfdwrdwnsdnscfcjzmrwqpsjwzrqcqcfndlvtfftbhdcbgcgtrfqrqnvwgbpstrtzgpjcjswfcwgvfnwlpprthntmlqbchjcwnqgppchgtvrzjbdzptfqqbmchmpqlnnfvpghjnmshpcjgsprbjpqnpwwddnswnfjvbzwszplhnhgzmzdqncmqrqtbqhdwbtrbrljpcwbdhjzvcqpdgvbtczlhwwzfmgvffnbcglpdqjdsnhhtnvvmtnhlbqcfqjfcgmcnqstzbgmqcfsgrzncwcfrlpsctpspvvdzwtbrhqzhfcwvwqvtzrmfjgpmlsdjmlgwhcldqlhsjvprvnmzcsldzfpzhmzdbqbpwnrsffswbjcwjgblnlcwzqlmcgfstqggdbsqpcpqcgfvn", 14)
