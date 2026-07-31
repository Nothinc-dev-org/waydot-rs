Name: waydot
Version: %{?version}%{!?version:0.1.0}
Release: 1%{?dist}
Summary: Expressive input panel with emojis, kaomojis, symbols and clipboard history
License: GPL-3.0-only
URL: https://github.com/Nothinc-dev-org/waydot-rs
Source0: waydot-%{version}.tar.gz

BuildRequires: cargo
BuildRequires: gtk4-devel
BuildRequires: libadwaita-devel
BuildRequires: openssl-devel
BuildRequires: rust
BuildRequires: wayland-devel

Requires: gtk4
Requires: libadwaita

%description
Waydot es un panel de entrada expresiva para escritorios Linux inspirado en el
panel Win+. de Windows 11: emojis, kaomojis, simbolos especiales e historial
del portapapeles. Se ejecuta en segundo plano y aparece en la bandeja del
sistema. Construido con Rust, GTK4 y Libadwaita.

%prep
%setup -q -n waydot-%{version}

%build
cargo build --release

%install
install -Dm0755 target/release/waydot %{buildroot}%{_bindir}/waydot
install -Dm0644 packaging/com.nothinc.waydot.desktop %{buildroot}%{_datadir}/applications/com.nothinc.waydot.desktop
install -Dm0644 packaging/com.nothinc.waydot.svg %{buildroot}%{_iconsdir}/hicolor/scalable/apps/com.nothinc.waydot.svg
install -Dm0644 packaging/com.nothinc.waydot.metainfo.xml %{buildroot}%{_datadir}/metainfo/com.nothinc.waydot.metainfo.xml

%files
%license LICENSE
%{_bindir}/waydot
%{_datadir}/applications/com.nothinc.waydot.desktop
%{_iconsdir}/hicolor/scalable/apps/com.nothinc.waydot.svg
%{_datadir}/metainfo/com.nothinc.waydot.metainfo.xml

%post
gtk-update-icon-cache %{_iconsdir}/hicolor &>/dev/null || :
update-desktop-database %{_datadir}/applications &>/dev/null || :

%postun
gtk-update-icon-cache %{_iconsdir}/hicolor &>/dev/null || :
update-desktop-database %{_datadir}/applications &>/dev/null || :

%changelog
* Fri Jul 31 2026 alcss <fernandoalcalacasas@hotmail.com> - 0.1.0-1
- Primera release empaquetada como RPM
