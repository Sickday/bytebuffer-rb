require 'bundler/setup'
require 'bundler/gem_tasks'
require 'ffi'
require 'rake'
require 'rake/testtask'
require 'rspec/core/rake_task'
require 'rubygems'
require 'rubygems/package_task'

namespace :gem do
  desc 'Build the Gem extensions'
  task :build do
    raise "Install rustc along with Cargo before running this rake task." if !system('cargo --version') || !system('rustc --version')

    FileUtils.chdir("ext/") do
      target = case RUBY_PLATFORM
               when /darwin/ then 'aarch64-apple-darwin'
               when /linux/ then 'x86_64-unknown-linux-gnu'
               when /mswin|mingw/ then 'x86_64-pc-windows-msvc'
               else 'x86_64-unknown-linux-gnu'
               end

      target_path = "./build/#{target}/release/#{RUBY_PLATFORM.match?(/mswin|mingw/) ? 'ext' : 'libext'}.#{FFI::Platform::LIBSUFFIX}"

      FileUtils.mkdir("build")

      system("cargo build --release --target=#{target} --target-dir=#{FileUtils.pwd}/build")

      FileUtils.cp(target_path, "./libext.#{FFI::Platform::LIBSUFFIX}")
    end
  end

  desc 'Clean up the Gem build environment.'
  task :clean do
    FileUtils.rm_rf('pkg/')
    FileUtils.rm_rf('ext/build/')
  end

  desc 'Package the Gem package'
  task :package do
    load('bytebuffer.gemspec')

    Gem::PackageTask.new(GEM_SPEC) do |pkg|
      pkg.need_tar_bz2 = true
    end

    Rake::Task['package'].invoke
  end
end

namespace :docs do
  require 'yard'

  YARD::Rake::YardocTask.new do |t|
    t.name = 'generate'
    t.files = ['lib/**/*.rb']
  end

  task serve: %i[docs:generate] do
    Kernel.system('yard server')
  end

  task :clean do
    require 'fileutils'

    FileUtils.rm_rf('doc')
    FileUtils.rm_rf('.yardoc')
  end
end


namespace :test do
  desc 'Runs all specs in the "spec/" folder using RSpec.'
  RSpec::Core::RakeTask.new(:rspec)

  task :clean do
    require 'fileutils'

    FileUtils.rm_rf('coverage')
  end

  desc 'Runs extension tests using cargo test.'
  task :ext do
    raise "Install rustc along with Cargo before running this rake task." if !system('cargo --version') || !system('rustc --version')

    FileUtils.chdir("ext/") do
      system("cargo test")
    end
  end
end