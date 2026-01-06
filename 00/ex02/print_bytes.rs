/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_bytes.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: dgaillet <dgaillet@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/01/05 14:16:55 by dgaillet          #+#    #+#             */
/*   Updated: 2026/01/05 14:45:58 by dgaillet         ###   ########lyon.fr   */
/*                                                                            */
/* ************************************************************************** */

fn print_bytes(s: &str) {
    let bytes = s.bytes();

    for n in bytes {
        println!("{}", n);
    }
}

fn main() {
    print_bytes("Déjà Vu\n");
}
